use crate::db::job_repo::JobRepo;
use crate::db::report_repo::ReportRepo;
use crate::services::{DocxService, PreviewService};
use crate::state::AppState;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub struct GenerationWorker;

impl GenerationWorker {
    pub async fn run(state: AppState) {
        tracing::info!("Generation worker started");
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

            match JobRepo::claim_generation_job(&state.db).await {
                Ok(Some((job_id, report_id, attempts))) => {
                    tracing::info!(job_id = %job_id, report_id = %report_id, attempts = %attempts, "Claimed generation job");
                    if let Err(e) = Self::process_job(&state, job_id, report_id).await {
                        tracing::error!(job_id = %job_id, error = ?e, "Generation job failed");
                        let _ = JobRepo::fail_generation_job(
                            &state.db,
                            job_id,
                            "GENERATION_FAILED",
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
                    tracing::error!("Error claiming generation job: {:?}", e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    async fn process_job(
        state: &AppState,
        job_id: Uuid,
        report_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 1. Fetch report details
        let (report, _) = ReportRepo::get_report_by_id(&state.db, report_id)
            .await?
            .ok_or("Report not found")?;

        let student = report.student.as_ref().ok_or("Missing student info")?;
        let internship = report
            .internship
            .as_ref()
            .ok_or("Missing internship info")?;
        let facts = report.research_facts.unwrap_or_default();

        // 2. Generate sections with LLM
        let sections = state
            .llm
            .generate_sections(&report.title, student, internship, &facts)
            .await?;

        // 3. Save generated sections
        ReportRepo::save_generated_sections(&state.db, report_id, &sections).await?;

        // 4. Load DOCX template (ensure default exists if missing)
        let template_path = state.storage.template_path();
        DocxService::ensure_default_template(&template_path)?;
        let template_bytes = state.storage.read_file(&template_path)?;

        // 5. Render DOCX with placeholders
        let docx_bytes = DocxService::render_report(
            &template_bytes,
            &report.title,
            &Some(student.clone()),
            &Some(internship.clone()),
            &Some(sections),
        )?;

        // 6. Write final DOCX to disk
        let file_uuid = Uuid::new_v4();
        let docx_out_path = state.storage.report_docx_path(report_id, file_uuid);
        state.storage.write_file(&docx_out_path, &docx_bytes)?;
        let docx_str = docx_out_path.to_str().unwrap_or_default().to_string();

        ReportRepo::save_file_entry(
            &state.db,
            report_id,
            "docx",
            &docx_str,
            docx_bytes.len() as i64,
        )
        .await?;

        // 7. Convert DOCX to PDF preview
        let preview_out_path = state.storage.report_preview_path(report_id, file_uuid);
        let scratch_dir = state.storage.temp_job_dir(job_id);

        PreviewService::convert_docx_to_pdf(&docx_out_path, &preview_out_path, &scratch_dir)
            .await?;
        let _ = state.storage.delete_dir(&scratch_dir);

        let preview_bytes = state.storage.read_file(&preview_out_path)?;
        let preview_str = preview_out_path.to_str().unwrap_or_default().to_string();

        ReportRepo::save_file_entry(
            &state.db,
            report_id,
            "preview",
            &preview_str,
            preview_bytes.len() as i64,
        )
        .await?;

        // 7.5 Optional: Stream to CDN if enabled
        if state.cdn.is_enabled() {
            let cdn_filename = format!("laporin_preview_{}.pdf", file_uuid);
            if let Ok(upload) = state.cdn.upload_bytes(&cdn_filename, preview_bytes.clone(), Some("application/pdf")).await {
                tracing::info!(report_id = %report_id, cdn_url = %upload.url, "Preview PDF published to Hack Club CDN");
            }
        }

        // 8. Complete job and update report status to preview_ready
        JobRepo::complete_generation_job(&state.db, job_id, &docx_str, &preview_str).await?;
        ReportRepo::update_status(&state.db, report_id, "preview_ready").await?;

        tracing::info!(job_id = %job_id, report_id = %report_id, "Generation job completed successfully");
        Ok(())
    }
}
