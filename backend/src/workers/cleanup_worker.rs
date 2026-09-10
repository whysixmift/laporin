use crate::db::session_repo::SessionRepo;
use crate::state::AppState;
use std::time::Duration;
use tokio::time::sleep;

pub struct CleanupWorker;

impl CleanupWorker {
    pub async fn run(state: AppState) {
        tracing::info!("Cleanup worker started");
        loop {
            sleep(Duration::from_secs(900)).await; // 15 minutes

            // 1. Purge expired sessions
            if let Ok(count) = SessionRepo::purge_expired(&state.db).await {
                if count > 0 {
                    tracing::info!(purged_sessions = %count, "Purged expired/revoked sessions");
                }
            }

            // 2. Clean temporary files older than 30 minutes in tmp directory
            let tmp_root = state
                .storage
                .temp_job_dir(uuid::Uuid::nil())
                .parent()
                .unwrap()
                .to_path_buf();
            if tmp_root.exists() {
                if let Ok(entries) = std::fs::read_dir(&tmp_root) {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                if let Ok(elapsed) = modified.elapsed() {
                                    if elapsed > Duration::from_secs(1800) {
                                        let _ = std::fs::remove_dir_all(entry.path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
