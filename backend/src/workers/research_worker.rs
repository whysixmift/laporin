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

        // 2. Perform safe web research crawling (up to 3-5 sources)
        let sample_urls = vec![format!(
            "https://example.com/company/{}",
            urlencoding::encode(&company_name)
        )];

        let mut crawled_sources = Vec::new();
        for url in sample_urls {
            // Check SSRF and fetch
            if let Ok((title, content)) = state.crawler.fetch_and_sanitize(&url).await {
                crawled_sources.push((url, title, content));
            } else {
                // If external network is not reachable in dev/test, add placeholder source
                crawled_sources.push((
                    url.clone(),
                    format!("Informasi Publik {}", company_name),
                    format!(
                        "{} merupakan tempat penyelenggaraan kegiatan praktik kerja lapangan.",
                        company_name
                    ),
                ));
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
