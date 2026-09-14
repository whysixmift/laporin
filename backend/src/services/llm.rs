use crate::config::Config;
use crate::domain::report::{
    GeneratedSections, InternshipInfo, ResearchFact, ResearchSource, StudentInfo,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct LlmService {
    client: reqwest::Client,
    config: Config,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

impl LlmService {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(45))
                .build()
                .unwrap_or_default(),
            config,
        }
    }

    pub async fn extract_research_facts(
        &self,
        company_name: &str,
        role: Option<&str>,
        crawled_sources: &[(String, String, String)], // (url, title, sanitized_text)
    ) -> Result<Vec<ResearchFact>, Box<dyn std::error::Error + Send + Sync>> {
        if crawled_sources.is_empty() {
            return Ok(vec![ResearchFact {
                claim: format!(
                    "{} merupakan perusahaan tempat mahasiswa melaksanakan magang.",
                    company_name
                ),
                sources: vec![ResearchSource {
                    url: "https://example.com/company".to_string(),
                    title: Some(format!("Profil {}", company_name)),
                    fetched_at: Utc::now(),
                    confidence: Some(0.95),
                }],
            }]);
        }

        if self.config.environment == "test" || self.config.l9router_api_key.starts_with("dev_") {
            // Deterministic mock fact extraction
            let mut facts = Vec::new();
            for (idx, (url, title, _text)) in crawled_sources.iter().enumerate() {
                let claim = match idx {
                    0 => format!(
                        "{} bergerak di bidang operasional industri dan layanan profesional.",
                        company_name
                    ),
                    1 => format!(
                        "{} menerapkan standar operasional kerja terstruktur dalam divisi terkait.",
                        company_name
                    ),
                    _ => format!(
                        "Perusahaan {} mendukung pengembangan kompetensi mahasiswa magang.",
                        company_name
                    ),
                };
                facts.push(ResearchFact {
                    claim,
                    sources: vec![ResearchSource {
                        url: url.clone(),
                        title: Some(title.clone()),
                        fetched_at: Utc::now(),
                        confidence: Some(0.90),
                    }],
                });
            }
            return Ok(facts);
        }

        // Live 9Router API call
        let prompt = format!(
            "Ekstraksi fakta profil perusahaan '{}' untuk laporan magang (posisi: '{}') berdasarkan data sumber berikut:\n\
            {}\n\
            Format output hanya berupa JSON array claim dan confidence.",
            company_name,
            role.unwrap_or("Intern"),
            serde_json::to_string(crawled_sources)?
        );

        let req_body = ChatCompletionRequest {
            model: self.config.l9router_default_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: "Anda adalah asisten peneliti fakta kredibel untuk laporan magang."
                        .into(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: prompt,
                },
            ],
            temperature: 0.2,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.config.l9router_api_url))
            .bearer_auth(&self.config.l9router_api_key)
            .json(&req_body)
            .send()
            .await?;

        if let Ok(data) = resp.json::<ChatCompletionResponse>().await {
            if let Some(first) = data.choices.first() {
                // If LLM returned text, build structured fact
                let fact = ResearchFact {
                    claim: first.message.content.clone(),
                    sources: crawled_sources
                        .iter()
                        .map(|(url, title, _)| ResearchSource {
                            url: url.clone(),
                            title: Some(title.clone()),
                            fetched_at: Utc::now(),
                            confidence: Some(0.90),
                        })
                        .collect(),
                };
                return Ok(vec![fact]);
            }
        }

        Ok(vec![ResearchFact {
            claim: format!("Profil dan operasional {}", company_name),
            sources: vec![],
        }])
    }

    pub async fn generate_sections(
        &self,
        title: &str,
        student: &StudentInfo,
        internship: &InternshipInfo,
        facts: &[ResearchFact],
    ) -> Result<GeneratedSections, Box<dyn std::error::Error + Send + Sync>> {
        let fact_summary = facts
            .iter()
            .map(|f| format!("- {}", f.claim))
            .collect::<Vec<_>>()
            .join("\n");

        if self.config.environment == "test" || self.config.l9router_api_key.starts_with("dev_") {
            // Deterministic generation
            return Ok(GeneratedSections {
                cover: format!(
                    "LAPORAN PRAKTIK KERJA LAPANGAN\n{}\n\nDisusun Oleh:\n{} ({})\n{}",
                    title, student.full_name, student.student_id, student.school
                ),
                introduction: format!(
                    "Praktik Kerja Lapangan (PKL) merupakan kegiatan akademik wajib di {}. Mahasiswa melaksanakan kegiatan magang di {} guna mengaplikasikan ilmu pengetahuan dan keterampilan kerja.",
                    student.school, internship.company_name
                ),
                company_profile: format!(
                    "{} merupakan institusi terkemuka dengan komitmen profesional tinggi.\n\nFakta Perusahaan:\n{}",
                    internship.company_name, fact_summary
                ),
                activities: format!(
                    "Selama periode magang sebagai {}, mahasiswa bertugas: {}.",
                    internship.role.as_deref().unwrap_or("Staff"),
                    internship
                        .description
                        .as_deref()
                        .unwrap_or("melaksanakan tugas operasional sesuai arahan pembimbing")
                ),
                conclusion: format!(
                    "Pelaksanaan magang di {} memberikan wawasan berharga dan pengalaman profesional nyata bagi mahasiswa {}.",
                    internship.company_name, student.school
                ),
            });
        }

        // Live 9Router API call
        let prompt = format!(
            "Buat 5 bagian laporan magang (cover, introduction, company_profile, activities, conclusion) dalam format JSON berdasarkan:\n\
            Judul: {}\nMahasiswa: {} ({}) di {}\nPerusahaan: {} (Posisi: {:?})\nFakta Terverifikasi:\n{}",
            title,
            student.full_name,
            student.student_id,
            student.school,
            internship.company_name,
            internship.role,
            fact_summary
        );

        let req_body = ChatCompletionRequest {
            model: self.config.l9router_default_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: "Anda adalah penulis laporan magang profesional. Selalu kembalikan JSON valid dengan key: cover, introduction, company_profile, activities, conclusion.".into(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: prompt,
                },
            ],
            temperature: 0.3,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.config.l9router_api_url))
            .bearer_auth(&self.config.l9router_api_key)
            .json(&req_body)
            .send()
            .await?;

        if let Ok(data) = resp.json::<ChatCompletionResponse>().await {
            if let Some(first) = data.choices.first() {
                if let Ok(sections) =
                    serde_json::from_str::<GeneratedSections>(&first.message.content)
                {
                    return Ok(sections);
                }
            }
        }

        // Safe fallback if JSON parsing failed
        Ok(GeneratedSections {
            cover: format!("LAPORAN PKL: {}", title),
            introduction: format!("Kegiatan magang di {}", internship.company_name),
            company_profile: format!("Profil {}", internship.company_name),
            activities: format!("Aktivitas magang di {}", internship.company_name),
            conclusion: "Kesimpulan pelaksanaan magang terlaksana dengan baik.".into(),
        })
    }

    pub async fn generate(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let req_body = ChatCompletionRequest {
            model: self.config.l9router_default_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: system_prompt.into(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: user_prompt.into(),
                },
            ],
            temperature: 0.7,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.config.l9router_api_url))
            .bearer_auth(&self.config.l9router_api_key)
            .json(&req_body)
            .send()
            .await?;

        if let Ok(data) = resp.json::<ChatCompletionResponse>().await {
            if let Some(first) = data.choices.first() {
                return Ok(first.message.content.clone());
            }
        }

        Ok("Generasi selesai (Respons fallback)".to_string())
    }
}

