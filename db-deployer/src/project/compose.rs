//! Docker Compose configuration modification
//!
//! Modifies the SERVICE_DB_URL environment variable in docker-compose.yml

use std::fs;
use std::path::Path;

use regex::Regex;
use tracing::info;

use crate::config::ProjectConfig;
use crate::error::{DeployError, Result};

/// Update the database URL in docker-compose.yml
pub fn update_db_url(
    project_config: &ProjectConfig,
    new_db_name: &str,
    db_host: &str,
    db_port: u16,
    db_user: &str,
    db_password: &str,
) -> Result<()> {
    let compose_path =
        Path::new(&project_config.project_dir).join(&project_config.docker_compose_file);

    info!(
        "Updating {} in docker-compose.yml...",
        project_config.db_url_env_name
    );

    if !compose_path.exists() {
        return Err(DeployError::Config(format!(
            "Docker Compose file not found: {}",
            compose_path.display()
        )));
    }

    // Read the file
    let content = fs::read_to_string(&compose_path).map_err(|e| {
        DeployError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to read docker-compose.yml: {}", e),
        ))
    })?;

    // Build the new database URL
    let new_db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, new_db_name
    );

    // Create regex pattern to match the env variable line
    // Match patterns like:
    //   SERVICE_DB_URL: postgres://...
    //   SERVICE_DB_URL=postgres://...
    let env_name = &project_config.db_url_env_name;
    let pattern = format!(r"({}:\s*)postgres://[^\s]+", regex::escape(env_name));

    let re = Regex::new(&pattern).map_err(|e| {
        DeployError::Config(format!("Invalid regex pattern: {}", e))
    })?;

    let new_content = if re.is_match(&content) {
        // Replace existing value
        re.replace(&content, format!("${{1}}{}", new_db_url))
            .to_string()
    } else {
        // Try alternate pattern with equals sign (for docker-compose environment array format)
        let pattern2 = format!(r"(- {}=)postgres://[^\s]+", regex::escape(env_name));
        let re2 = Regex::new(&pattern2).map_err(|e| {
            DeployError::Config(format!("Invalid regex pattern: {}", e))
        })?;

        if re2.is_match(&content) {
            re2.replace(&content, format!("${{1}}{}", new_db_url))
                .to_string()
        } else {
            return Err(DeployError::Config(format!(
                "{} not found in docker-compose.yml. Please add it manually.",
                env_name
            )));
        }
    };

    // Write back the modified content
    fs::write(&compose_path, new_content).map_err(|e| {
        DeployError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write docker-compose.yml: {}", e),
        ))
    })?;

    info!("  Updated {} to use database: {}", env_name, new_db_name);
    info!(
        "  NOTE: This change is local only. Do NOT commit this change to git."
    );

    Ok(())
}

/// Get current database URL from docker-compose.yml (for display/backup purposes)
pub fn get_current_db_url(project_config: &ProjectConfig) -> Result<Option<String>> {
    let compose_path =
        Path::new(&project_config.project_dir).join(&project_config.docker_compose_file);

    if !compose_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&compose_path).map_err(|e| {
        DeployError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to read docker-compose.yml: {}", e),
        ))
    })?;

    let env_name = &project_config.db_url_env_name;

    // Try to extract the current value
    let pattern = format!(r"{}:\s*(postgres://[^\s]+)", regex::escape(env_name));
    let re = Regex::new(&pattern).ok();

    if let Some(re) = re {
        if let Some(caps) = re.captures(&content) {
            return Ok(caps.get(1).map(|m| m.as_str().to_string()));
        }
    }

    // Try alternate pattern
    let pattern2 = format!(r"- {}=(postgres://[^\s]+)", regex::escape(env_name));
    let re2 = Regex::new(&pattern2).ok();

    if let Some(re2) = re2 {
        if let Some(caps) = re2.captures(&content) {
            return Ok(caps.get(1).map(|m| m.as_str().to_string()));
        }
    }

    Ok(None)
}
