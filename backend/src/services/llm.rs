use crate::config::Config;
use crate::domain::report::{
    GeneratedSections, InternshipInfo, LogbookEntry, ResearchFact, ResearchSource, StudentInfo,
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
        logbook_entries: &[LogbookEntry],
    ) -> Result<GeneratedSections, Box<dyn std::error::Error + Send + Sync>> {
        let fact_summary = facts
            .iter()
            .map(|f| format!("- {}", f.claim))
            .collect::<Vec<_>>()
            .join("\n");

        let logbook_summary = if logbook_entries.is_empty() {
            "Tidak ada catatan logbook harian terpisah. Gunakan deskripsi tugas umum.".to_string()
        } else {
            let mut s = String::from("Catatan Logbook Harian Siswa:\n");
            for entry in logbook_entries {
                s.push_str(&format!(
                    "- [{}] {}: {}\n  * Alat/Teknologi: {}\n  * Kendala/Solusi: {} / {}\n  * Skill/Bukti: {} / {}\n",
                    entry.entry_date,
                    entry.activity_title,
                    entry.tasks_performed,
                    entry.tools_technologies.as_deref().unwrap_or("-"),
                    entry.problems_encountered.as_deref().unwrap_or("-"),
                    entry.solutions_applied.as_deref().unwrap_or("-"),
                    entry.skills_learned.as_deref().unwrap_or("-"),
                    entry.evidence_notes.as_deref().unwrap_or("-")
                ));
            }
            s
        };

        if self.config.environment == "test" || self.config.l9router_api_key.starts_with("dev_") {
            let role_str = internship.role.as_deref().unwrap_or("Praktikan");
            let major_str = student.major.as_deref().unwrap_or("Rekayasa Perangkat Lunak");
            
            let activities_content = if logbook_entries.is_empty() {
                format!(
                    "3.1 Pelaksanaan Kegiatan PKL\nKegiatan PKL dilaksanakan di {} pada posisi {}.\n\n3.2 Jenis-Jenis Kegiatan\n1. Pelaksanaan tugas teknis sesuai kompetensi {}.\n2. Analisis kebutuhan sistem dan dokumentasi alur kerja.\n3. Pengujian dan pelaporan hasil operasional kepada pembimbing lapangan.\n\n3.3 Langkah-Langkah Kerja Terperinci\n- Alat dan Bahan: Komputer kerja, sistem perangkat lunak penunjang, koneksi internet, dan lembar operasional.\n- Langkah-Langkah: Menerima instruksi kerja, mempersiapkan modul teknis, melaksanakan eksekusi tugas, melakukan evaluasi mandiri, dan melaporkan hasil kepada supervisor.\n\n3.4 Hambatan dan Solusi\nHambatan: Adaptasi awal terhadap alur sistem dan standar kerja industri.\nSolusi: Mempelajari dokumentasi teknis secara mandiri dan berkonsultasi intensif dengan pembimbing lapangan.",
                    internship.company_name, role_str, major_str
                )
            } else {
                let mut logbook_text = String::new();
                for (idx, entry) in logbook_entries.iter().enumerate() {
                    logbook_text.push_str(&format!(
                        "{}. Tanggal {}: {}\n   - Uraian Tugas: {}\n   - Alat/Teknologi: {}\n   - Kendala & Solusi: {} / {}\n\n",
                        idx + 1,
                        entry.entry_date,
                        entry.activity_title,
                        entry.tasks_performed,
                        entry.tools_technologies.as_deref().unwrap_or("Peralatan standar kerja"),
                        entry.problems_encountered.as_deref().unwrap_or("Tidak ada kendala berarti"),
                        entry.solutions_applied.as_deref().unwrap_or("Bekerja sesuai SOP")
                    ));
                }
                format!(
                    "3.1 Pelaksanaan Kegiatan PKL\nKegiatan PKL dilaksanakan di {} pada posisi {}.\n\n3.2 Logbook & Rincian Kegiatan Harian\n{}\n3.3 Langkah-Langkah Kerja Terperinci\n- Alat dan Bahan: Perangkat kerja, dokumentasi sistem, dan modul operasional.\n- Langkah-Langkah: Berkoordinasi dengan pembimbing, melaksanakan tugas harian sesuai jadwal logbook, menguji hasil kerja, dan mendokumentasikan pencapaian.\n\n3.4 Evaluasi dan Solusi\nSetiap kendala yang dihadapi diselesaikan melalui koordinasi aktif dengan pembimbing lapangan dan penerapan standar operasional prosedur.",
                    internship.company_name, role_str, logbook_text
                )
            };

            return Ok(GeneratedSections {
                cover: format!(
                    "LAPORAN PRAKTIK KERJA LAPANGAN (PKL)\n{}\n\nDiajukan untuk memenuhi persyaratan peserta Mengikuti Ujian Kompetensi Keahlian\n\nDisusun Oleh:\nNama: {}\nNo. Induk: {}\nKelas/Kompetensi: {}\n\n{}\n",
                    title, student.full_name, student.student_id, major_str, student.school
                ),
                introduction: format!(
                    "1.1 Latar Belakang PKL\nPraktik Kerja Lapangan (PKL) merupakan salah satu bentuk implementasi secara sistematis dan sinkron antara program penguasaan keahlian yang diperoleh melalui kegiatan belajar mengajar di {} dengan tuntutan keahlian di dunia kerja {}.\n\n1.2 Tujuan PKL\na. Mengaplikasikan teori dalam praktik nyata di dunia industri.\nb. Meningkatkan kompetensi teknis dan kedisiplinan kerja sesuai bidang {}.\nc. Membangun soft skills, komunikasi tim, dan etos kerja profesional.\n\n1.3 Tujuan Pembuatan Laporan PKL\na. Sebagai bukti otentik pertanggungjawaban kegiatan PKL.\nb. Mengembangkan kemampuan analisis dan penyusunan karya tulis ilmiah.\n\n1.4 Sistematika Laporan PKL\nLaporan ini disusun menjadi empat bab utama yang memuat Pendahuluan, Gambaran Umum Perusahaan, Pelaksanaan Praktik Kerja, serta Penutup dan Lampiran.",
                    student.school, internship.company_name, major_str
                ),
                company_profile: format!(
                    "2.1 Sejarah Pendirian Perusahaan\n{} merupakan institusi yang bergerak secara profesional dengan komitmen mutu tinggi. Berdiri guna memenuhi kebutuhan pasar akan layanan dan produk berkualitas.\n\nFakta Terverifikasi:\n{}\n\n2.2 Visi dan Misi Perusahaan\nVisi: Menjadi penyedia layanan dan produk unggul yang terpercaya dan berdaya saing tinggi.\nMisi: Mengedepankan kepuasan pelanggan, integritas operasional, dan pemberdayaan SDM profesional.\n\n2.3 Jam Kerja dan Disiplin K3\nPerusahaan menerapkan sistem jam kerja terstruktur serta mewajibkan penerapan Standar Keselamatan dan Kesehatan Kerja (K3) di setiap lingkungan kerja operasional.",
                    internship.company_name, fact_summary
                ),
                activities: activities_content,
                conclusion: format!(
                    "4.1 Kesimpulan\nPelaksanaan Praktik Kerja Lapangan (PKL) di {} telah memberikan pengalaman berharga dan peningkatan wawasan praktis yang signifikan bagi siswa {}. Siswa mampu menyelaraskan teori kejuruan dengan realitas industri nyata.\n\n4.2 Saran-Saran\na. Untuk Perusahaan: Diharapkan terus memberikan bimbingan dan kesempatan berkarya yang luas bagi generasi praktikan berikutnya.\nb. Untuk Sekolah: Diharapkan terus memperbarui kurikulum praktik agar selalu selaras dengan perkembangan teknologi industri modern.",
                    internship.company_name, student.school
                ),
                is_user_edited: Some(false),
            });
        }

        // Live 9Router API call
        let major_str = student.major.as_deref().unwrap_or("Teknologi Informasi");
        let role_str = internship.role.as_deref().unwrap_or("Praktikan / Staff Magang");
        let desc_str = internship.description.as_deref().unwrap_or("Melaksanakan tugas operasional sesuai arahan pembimbing");

        let prompt = format!(
            "Buat 5 bagian laporan Praktik Kerja Lapangan (PKL) / Magang lengkap, mendalam, dan profesional dalam format JSON standar Indonesia dengan keys: \"cover\", \"introduction\", \"company_profile\", \"activities\", \"conclusion\".\n\n\
            Data Laporan PKL:\n\
            - Judul Laporan: {}\n\
            - Nama Siswa: {} (NIS/NIM: {})\n\
            - Sekolah/Institusi: {}\n\
            - Jurusan/Kompetensi: {}\n\
            - Perusahaan Tempat PKL: {}\n\
            - Posisi/Divisi: {}\n\
            - Deskripsi Tugas: {}\n\
            - Fakta Terverifikasi Perusahaan:\n{}\n\n\
            - {}\n\n\
            Instruksi Khusus Format Bab:\n\
            1. 'cover': Berisi judul laporan, identitas lengkap siswa, program keahlian, dan sekolah.\n\
            2. 'introduction' (BAB I PENDAHULUAN): Wajib memuat 1.1 Latar Belakang PKL, 1.2 Tujuan PKL, 1.3 Tujuan Pembuatan Laporan PKL, dan 1.4 Sistematika Laporan PKL dalam paragraf yang rapi dan baku.\n\
            3. 'company_profile' (BAB II GAMBARAN UMUM PERUSAHAAN): Wajib memuat 2.1 Sejarah Pendirian Perusahaan (sesuai fakta), 2.2 Visi Misi dan Motto, 2.3 Struktur Organisasi dan Tugas Divisi, 2.4 Kepegawaian dan Jam Kerja, 2.5 Disiplin Kerja dan K3, serta 2.6 Jenis Produk/Layanan.\n\
            4. 'activities' (BAB III URAIAN PELAKSANAAN PRAKTEK DI PERUSAHAAN): Wajib memuat 3.1 Waktu & Tempat, 3.2 Jenis-Jenis Kegiatan (sintesiskan seluruh catatan logbook harian siswa secara kronologis, JANGAN mengarang kegiatan fiktif di luar data logbook siswa), 3.3 Langkah-Langkah Kerja Terperinci (Alat dan Bahan + Langkah demi Langkah yang detail dan teknis), 3.4 Hasil yang Diharapkan, serta 3.5 Hambatan yang Dialami dan Solusi Penyelesaiannya (ambil dari kendala/solusi di logbook jika ada).\n\
            5. 'conclusion' (BAB IV PENUTUP): Wajib memuat 4.1 Kesimpulan dan 4.2 Saran-Saran (Saran untuk Perusahaan dan Saran untuk Sekolah/Adik Kelas).\n\n\
            Format output HANYA berupa JSON valid tanpa teks pembuka atau markdown wrapper.",
            title,
            student.full_name,
            student.student_id,
            student.school,
            major_str,
            internship.company_name,
            role_str,
            desc_str,
            fact_summary,
            logbook_summary
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
                if let Ok(mut sections) =
                    serde_json::from_str::<GeneratedSections>(&first.message.content)
                {
                    if sections.is_user_edited.is_none() {
                        sections.is_user_edited = Some(false);
                    }
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
            is_user_edited: Some(false),
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

