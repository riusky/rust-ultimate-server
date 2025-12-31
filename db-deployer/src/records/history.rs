use crate::config::Config;
use crate::error::Result;
use crate::records::CurrentRecord;
use chrono::Utc;
use std::fs;
use std::path::Path;

/// Archive current record to history
pub fn archive_current(config: &Config) -> Result<()> {
    let records_dir = Path::new(&config.paths.records_dir);
    let history_dir = records_dir.join("history");
    fs::create_dir_all(&history_dir)?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let current_path = records_dir.join("current.json");
    let sig_path = records_dir.join("current.json.sig");

    if current_path.exists() {
        let archive_name = format!("{}.json", timestamp);
        let archive_path = history_dir.join(&archive_name);
        fs::copy(&current_path, &archive_path)?;
    }

    if sig_path.exists() {
        let archive_sig_name = format!("{}.json.sig", timestamp);
        let archive_sig_path = history_dir.join(&archive_sig_name);
        fs::copy(&sig_path, &archive_sig_path)?;
    }

    Ok(())
}

/// Clean up old databases based on retain_versions
pub fn get_databases_to_cleanup(record: &CurrentRecord) -> Vec<String> {
    let retain = record.retain_versions;
    if record.history.len() <= retain {
        return Vec::new();
    }

    let to_remove = record.history.len() - retain;
    record
        .history
        .iter()
        .take(to_remove)
        .map(|h| h.db_name.clone())
        .collect()
}

/// List archived records
pub fn list_archives(config: &Config) -> Result<Vec<String>> {
    let history_dir = Path::new(&config.paths.records_dir).join("history");
    if !history_dir.exists() {
        return Ok(Vec::new());
    }

    let mut archives = Vec::new();
    for entry in fs::read_dir(&history_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                archives.push(name.to_string());
            }
        }
    }

    archives.sort();
    Ok(archives)
}
