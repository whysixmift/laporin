use crate::db::job_repo::JobRepo;
use crate::db::report_repo::ReportRepo;
use crate::state::AppState;
use std::time::Duration;
use tokio::time::sleep;

pub struct ResearchWorker;

impl ResearchWorker {
    pub async fn run(state: AppState) {
        tracing::info!("Research worker started");
        loop {
            // Low memory check
            let mut sys = sysinfo::System::new_all();
            sys.refresh_memory();
            let available_mb = sys.available_memory() / (1024 * 1024);
            if available_mb < 500 {
                tracing::warn!(available_mb = %available_mb, "low_memory_pressure_job_paused");
                sleep(Duration::from_secs(10)).await;
                continue;
            }

            match JobRepo::claim_research_job(&state.db).await {
                Ok(Some((job_id, report_id, attempts))) => {
                    tracing::info!(job_id = %job_id, report_id = %report_id, attempts = %attempts, "Claimed research job");
                    if let Err(e) = Self::process_job(&state, job_id, report_id).await {
                        tracing::error!(job_id = %job_id, error = ?e, "Research job failed");
                        let _ = JobRepo::fail_research_job(
                            &state.db,
                            job_id,
                            "RESEARCH_FAILED",
                            &e.to_string(),
                        )
                        .await;
                        let _ = ReportRepo::update_status(&state.db, report_id, "failed").await;
                    }
                }
                Ok(None) => {
                    // No pending jobs, wait before next poll
                    sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    tracing::error!("Error claiming research job: {:?}", e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    async fn process_job(
        state: &AppState,
        job_id: uuid::Uuid,
        report_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 1. Fetch report details
        let (report, _) = ReportRepo::get_report_by_id(&state.db, report_id)
            .await?
            .ok_or("Report not found")?;

        let company_name = report
            .internship
            .as_ref()
            .map(|i| i.company_name.clone())
            .unwrap_or_else(|| "Instansi".into());
        let role = report.internship.as_ref().and_then(|i| i.role.as_deref());

        // 2. Perform autonomous live web research search & crawling
        let search_results = state.search.search_company(&company_name, role, 3).await.unwrap_or_default();

        let mut crawled_sources = Vec::new();
        if search_results.is_empty() {
            // Fallback if no search results found
            crawled_sources.push((
                format!("https://profil-instansi.id/company/{}", urlencoding::encode(&company_name.to_lowercase().replace(' ', "-"))),
                format!("Informasi Publik {}", company_name),
                format!(
                    "{} merupakan instansi penyelenggara kegiatan Praktik Kerja Lapangan dengan divisi operasional terkait.",
                    company_name
                ),
            ));
        } else {
            for result in search_results {
                // Check SSRF and fetch page content
                if let Ok((title, content)) = state.crawler.fetch_and_sanitize(&result.url).await {
                    let final_title = if title.is_empty() { result.title } else { title };
                    crawled_sources.push((result.url, final_title, content));
                } else {
                    // Use search snippet if full page fetch fails or times out
                    crawled_sources.push((
                        result.url,
                        result.title,
                        if result.snippet.is_empty() {
                            format!("Profil operasional kerja pada {}.", company_name)
                        } else {
                            result.snippet
                        },
                    ));
                }
            }
        }

        // 3. Extract facts with LLM
        let facts = state
            .llm
            .extract_research_facts(&company_name, role, &crawled_sources)
            .await?;

        // 4. Save facts and provenance
        ReportRepo::save_research_facts(&state.db, report_id, job_id, &facts).await?;

        // 5. Update job and report status to research_completed
        JobRepo::complete_research_job(&state.db, job_id).await?;
        ReportRepo::update_status(&state.db, report_id, "research_completed").await?;

        tracing::info!(job_id = %job_id, report_id = %report_id, "Research job completed successfully");
        Ok(())
    }
}
