use crate::config::Config;
use crate::crypto;
use crate::error::{DeployError, Result};
use crate::records::CurrentRecord;
use std::fs;
use std::path::{Path, PathBuf};

/// Get the path to current.json
pub fn current_json_path(config: &Config) -> PathBuf {
    Path::new(&config.paths.records_dir).join("current.json")
}

/// Get the path to current.json.sig
pub fn current_sig_path(config: &Config) -> PathBuf {
    Path::new(&config.paths.records_dir).join("current.json.sig")
}

/// Check if current.json exists
pub fn exists(config: &Config) -> bool {
    current_json_path(config).exists()
}

/// Load current.json
pub fn load(config: &Config) -> Result<CurrentRecord> {
    let path = current_json_path(config);
    if !path.exists() {
        return Err(DeployError::RecordNotFound);
    }

    let content = fs::read_to_string(&path)?;
    let record: CurrentRecord = serde_json::from_str(&content)
        .map_err(|e| DeployError::RecordCorrupted(e.to_string()))?;

    Ok(record)
}

/// Load and verify current.json signature
pub fn load_verified(config: &Config) -> Result<CurrentRecord> {
    let data_path = current_json_path(config);
    let sig_path = current_sig_path(config);
    let public_key_path = Path::new(&config.keys.public_key);

    if !sig_path.exists() {
        return Err(DeployError::SignatureNotFound(sig_path.display().to_string()));
    }

    let is_valid = crypto::verify_file(public_key_path, &data_path, &sig_path)?;
    if !is_valid {
        return Err(DeployError::SignatureInvalid);
    }

    load(config)
}

/// Save current.json and sign it
pub fn save(config: &Config, record: &CurrentRecord) -> Result<()> {
    let records_dir = Path::new(&config.paths.records_dir);
    fs::create_dir_all(records_dir)?;

    let data_path = current_json_path(config);
    let sig_path = current_sig_path(config);
    let private_key_path = Path::new(&config.keys.private_key);

    // Save JSON
    let content = serde_json::to_string_pretty(record)
        .map_err(|e| DeployError::Other(format!("Failed to serialize record: {}", e)))?;
    fs::write(&data_path, &content)?;

    // Sign
    crypto::sign_file(private_key_path, &data_path, &sig_path)?;

    Ok(())
}

/// Get all SQL file records (init + updates)
pub fn get_all_sql_records(record: &CurrentRecord) -> Vec<&crate::records::SqlFileRecord> {
    record
        .init_files
        .iter()
        .chain(record.update_files.iter())
        .collect()
}
