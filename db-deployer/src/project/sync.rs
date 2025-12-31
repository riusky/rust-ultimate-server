//! SQL file synchronization
//!
//! Copies SQL files from project repository to deployment tool directory

use std::fs;
use std::path::Path;

use tracing::info;

use crate::config::{Config, ProjectConfig};
use crate::error::{DeployError, Result};

/// Sync SQL files from project repository to deployment tool
pub fn sync_sql_files(config: &Config, project_config: &ProjectConfig) -> Result<()> {
    let source_dir = Path::new(&project_config.project_dir).join(&project_config.sql_source_dir);

    info!(
        "Syncing SQL files from {} to local...",
        source_dir.display()
    );

    if !source_dir.exists() {
        return Err(DeployError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "SQL source directory not found in project: {}",
                source_dir.display()
            ),
        )));
    }

    // Sync init SQL directory
    let source_init = source_dir.join("init");
    let target_init = Path::new(&config.paths.init_sql_dir);

    if source_init.exists() {
        sync_directory(&source_init, target_init)?;
        info!(
            "  Synced init SQL: {} -> {}",
            source_init.display(),
            target_init.display()
        );
    }

    // Sync updates SQL directory
    let source_updates = source_dir.join("updates");
    let target_updates = Path::new(&config.paths.updates_sql_dir);

    if source_updates.exists() {
        sync_directory(&source_updates, target_updates)?;
        info!(
            "  Synced updates SQL: {} -> {}",
            source_updates.display(),
            target_updates.display()
        );
    }

    // If neither exists, try to sync the root sql dir as updates
    if !source_init.exists() && !source_updates.exists() {
        // Just copy all .sql files from source_dir to updates
        sync_sql_files_flat(&source_dir, target_updates)?;
        info!(
            "  Synced SQL files: {} -> {}",
            source_dir.display(),
            target_updates.display()
        );
    }

    Ok(())
}

/// Sync a directory by copying all files
fn sync_directory(source: &Path, target: &Path) -> Result<()> {
    // Create target directory if it doesn't exist
    if !target.exists() {
        fs::create_dir_all(target).map_err(|e| {
            DeployError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create directory {}: {}", target.display(), e),
            ))
        })?;
    }

    // Clear target directory first (remove old files)
    for entry in fs::read_dir(target).map_err(DeployError::Io)? {
        let entry = entry.map_err(DeployError::Io)?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
            fs::remove_file(&path).map_err(DeployError::Io)?;
        }
    }

    // Copy all SQL files from source
    for entry in fs::read_dir(source).map_err(DeployError::Io)? {
        let entry = entry.map_err(DeployError::Io)?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
            let filename = path.file_name().unwrap();
            let target_file = target.join(filename);

            fs::copy(&path, &target_file).map_err(|e| {
                DeployError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to copy {} to {}: {}",
                        path.display(),
                        target_file.display(),
                        e
                    ),
                ))
            })?;
        }
    }

    Ok(())
}

/// Sync SQL files from a flat directory structure
fn sync_sql_files_flat(source: &Path, target: &Path) -> Result<()> {
    // Create target directory if it doesn't exist
    if !target.exists() {
        fs::create_dir_all(target).map_err(|e| {
            DeployError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create directory {}: {}", target.display(), e),
            ))
        })?;
    }

    // Copy all SQL files from source
    let mut copied = 0;
    for entry in fs::read_dir(source).map_err(DeployError::Io)? {
        let entry = entry.map_err(DeployError::Io)?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
            let filename = path.file_name().unwrap();
            let target_file = target.join(filename);

            fs::copy(&path, &target_file).map_err(|e| {
                DeployError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to copy {} to {}: {}",
                        path.display(),
                        target_file.display(),
                        e
                    ),
                ))
            })?;
            copied += 1;
        }
    }

    info!("  Copied {} SQL files", copied);
    Ok(())
}
