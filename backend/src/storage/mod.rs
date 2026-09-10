use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub root_dir: PathBuf,
}

impl StorageConfig {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }
}

#[derive(Clone)]
pub struct Storage {
    config: StorageConfig,
}

impl Storage {
    pub fn new(config: StorageConfig) -> Self {
        Self { config }
    }

    pub fn ensure_dirs(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.config.root_dir).map_err(StorageError::Io)?;
        fs::create_dir_all(self.config.root_dir.join("templates")).map_err(StorageError::Io)?;
        fs::create_dir_all(self.config.root_dir.join("reports")).map_err(StorageError::Io)?;
        fs::create_dir_all(self.config.root_dir.join("tmp")).map_err(StorageError::Io)?;
        Ok(())
    }

    pub fn resolve_path(&self, relative: &Path) -> Result<PathBuf, StorageError> {
        let root = fs::canonicalize(&self.config.root_dir)
            .unwrap_or_else(|_| self.config.root_dir.clone());
        let target = root.join(relative);

        // For non-existent files yet, check parent directory canonical path
        if let Some(parent) = target.parent() {
            if parent.exists() {
                let canonical_parent = fs::canonicalize(parent).map_err(StorageError::Io)?;
                if !canonical_parent.starts_with(&root) {
                    return Err(StorageError::PathTraversal);
                }
            }
        }

        if target.exists() {
            let canonical_target = fs::canonicalize(&target).map_err(StorageError::Io)?;
            if !canonical_target.starts_with(&root) {
                return Err(StorageError::PathTraversal);
            }
            return Ok(canonical_target);
        }

        Ok(target)
    }

    pub fn template_path(&self) -> PathBuf {
        self.config
            .root_dir
            .join("templates")
            .join("report-template-v1.docx")
    }

    pub fn report_docx_path(&self, report_id: Uuid, file_uuid: Uuid) -> PathBuf {
        self.config
            .root_dir
            .join("reports")
            .join(report_id.to_string())
            .join("docx")
            .join(format!("{}.docx", file_uuid))
    }

    pub fn report_preview_path(&self, report_id: Uuid, file_uuid: Uuid) -> PathBuf {
        self.config
            .root_dir
            .join("reports")
            .join(report_id.to_string())
            .join("preview")
            .join(format!("{}.pdf", file_uuid))
    }

    pub fn temp_job_dir(&self, job_id: Uuid) -> PathBuf {
        self.config.root_dir.join("tmp").join(job_id.to_string())
    }

    pub fn write_file(&self, path: &Path, content: &[u8]) -> Result<(), StorageError> {
        let root = fs::canonicalize(&self.config.root_dir)
            .unwrap_or_else(|_| self.config.root_dir.clone());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(StorageError::Io)?;
            let canonical_parent = fs::canonicalize(parent).map_err(StorageError::Io)?;
            if !canonical_parent.starts_with(&root) {
                return Err(StorageError::PathTraversal);
            }
        }
        fs::write(path, content).map_err(StorageError::Io)?;
        Ok(())
    }

    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>, StorageError> {
        let root = fs::canonicalize(&self.config.root_dir)
            .unwrap_or_else(|_| self.config.root_dir.clone());
        if !path.exists() {
            return Err(StorageError::NotFound);
        }
        let canonical = fs::canonicalize(path).map_err(StorageError::Io)?;
        if !canonical.starts_with(&root) {
            return Err(StorageError::PathTraversal);
        }
        fs::read(&canonical).map_err(StorageError::Io)
    }

    pub fn file_exists(&self, path: &Path) -> bool {
        if !path.exists() {
            return false;
        }
        let root = fs::canonicalize(&self.config.root_dir)
            .unwrap_or_else(|_| self.config.root_dir.clone());
        if let Ok(canonical) = fs::canonicalize(path) {
            canonical.starts_with(&root) && canonical.is_file()
        } else {
            false
        }
    }

    pub fn delete_file(&self, path: &Path) -> Result<(), StorageError> {
        if path.exists() {
            let root = fs::canonicalize(&self.config.root_dir)
                .unwrap_or_else(|_| self.config.root_dir.clone());
            let canonical = fs::canonicalize(path).map_err(StorageError::Io)?;
            if !canonical.starts_with(&root) {
                return Err(StorageError::PathTraversal);
            }
            fs::remove_file(canonical).map_err(StorageError::Io)?;
        }
        Ok(())
    }

    pub fn delete_dir(&self, path: &Path) -> Result<(), StorageError> {
        if path.exists() {
            let root = fs::canonicalize(&self.config.root_dir)
                .unwrap_or_else(|_| self.config.root_dir.clone());
            let canonical = fs::canonicalize(path).map_err(StorageError::Io)?;
            if !canonical.starts_with(&root) {
                return Err(StorageError::PathTraversal);
            }
            fs::remove_dir_all(canonical).map_err(StorageError::Io)?;
        }
        Ok(())
    }

    pub fn check_available_disk_space(&self) -> u64 {
        // Return available space in bytes on storage root mount
        let sys = sysinfo::Disks::new_with_refreshed_list();
        for disk in &sys {
            if self.config.root_dir.starts_with(disk.mount_point()) {
                return disk.available_space();
            }
        }
        // Default to a safe large value if not detectable
        10 * 1024 * 1024 * 1024
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path resolution failed")]
    PathResolution,
    #[error("Path traversal attempt detected")]
    PathTraversal,
    #[error("File not found")]
    NotFound,
}
