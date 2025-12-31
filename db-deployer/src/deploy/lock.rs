use crate::config::Config;
use crate::error::{DeployError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const LOCK_FILE_NAME: &str = "deploy.lock";

/// Lock file content
#[derive(Debug, Serialize, Deserialize)]
pub struct LockInfo {
    pub pid: u32,
    pub started_at: DateTime<Utc>,
    pub hostname: String,
}

/// Deployment lock guard
pub struct DeployLock {
    path: PathBuf,
}

impl DeployLock {
    /// Acquire deployment lock
    pub fn acquire(config: &Config) -> Result<Self> {
        let records_dir = Path::new(&config.paths.records_dir);
        fs::create_dir_all(records_dir)?;

        let lock_path = records_dir.join(LOCK_FILE_NAME);

        if lock_path.exists() {
            // Check if lock is stale (optional: check if process is alive)
            let content = fs::read_to_string(&lock_path)?;
            if serde_json::from_str::<LockInfo>(&content).is_ok() {
                return Err(DeployError::DeploymentLocked);
            }
            // If lock file is corrupted, remove it
            fs::remove_file(&lock_path)?;
        }

        let info = LockInfo {
            pid: std::process::id(),
            started_at: Utc::now(),
            hostname: hostname::get()
                .map(|h| h.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
        };

        let content = serde_json::to_string_pretty(&info)
            .map_err(|e| DeployError::LockAcquireFailed(e.to_string()))?;

        fs::write(&lock_path, content)?;

        Ok(DeployLock { path: lock_path })
    }

    /// Check if locked
    pub fn is_locked(config: &Config) -> bool {
        let records_dir = Path::new(&config.paths.records_dir);
        records_dir.join(LOCK_FILE_NAME).exists()
    }

    /// Get lock info
    pub fn get_lock_info(config: &Config) -> Option<LockInfo> {
        let records_dir = Path::new(&config.paths.records_dir);
        let lock_path = records_dir.join(LOCK_FILE_NAME);

        if !lock_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&lock_path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Release the lock
    pub fn release(self) -> Result<()> {
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}

impl Drop for DeployLock {
    fn drop(&mut self) {
        // Automatically release lock when dropped
        if self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}
