//! Docker operations for project deployment
//!
//! Handles:
//! - Stopping services via docker-compose
//! - Starting services via docker-compose

use std::path::Path;
use std::process::Command;

use tracing::info;

use crate::config::ProjectConfig;
use crate::error::{DeployError, Result};

/// Get the full path to the docker-compose file
pub fn get_compose_path(project_config: &ProjectConfig) -> std::path::PathBuf {
    Path::new(&project_config.project_dir).join(&project_config.docker_compose_file)
}

/// Stop the web-server service using docker-compose
pub fn stop_service(project_config: &ProjectConfig) -> Result<()> {
    let compose_path = get_compose_path(project_config);

    info!(
        "Stopping service '{}' via docker-compose...",
        project_config.service_name
    );

    if !compose_path.exists() {
        return Err(DeployError::Config(format!(
            "Docker Compose file not found: {}",
            compose_path.display()
        )));
    }

    // Try docker compose (new syntax) first, fall back to docker-compose
    let result = try_docker_compose_stop(&compose_path, &project_config.service_name);

    match result {
        Ok(_) => {
            info!("  Service stopped");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Try to stop using docker compose (new or old syntax)
fn try_docker_compose_stop(compose_path: &Path, service_name: &str) -> Result<()> {
    // Try new syntax: docker compose
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_path.to_str().unwrap_or_default(),
            "stop",
            service_name,
        ])
        .output();

    match output {
        Ok(o) if o.status.success() => return Ok(()),
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            // If new syntax fails, try old syntax
            if stderr.contains("is not a docker command")
                || stderr.contains("unknown command")
            {
                return try_docker_compose_legacy(compose_path, service_name, "stop");
            }
            return Err(DeployError::Docker(format!(
                "Failed to stop service: {}",
                stderr
            )));
        }
        Err(_) => {
            // docker command not found, try docker-compose
            return try_docker_compose_legacy(compose_path, service_name, "stop");
        }
    }
}

/// Try legacy docker-compose command
fn try_docker_compose_legacy(compose_path: &Path, service_name: &str, action: &str) -> Result<()> {
    let output = Command::new("docker-compose")
        .args(["-f", compose_path.to_str().unwrap_or_default(), action, service_name])
        .output()
        .map_err(|e| DeployError::Docker(format!("Failed to run docker-compose: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Docker(format!(
            "docker-compose {} failed: {}",
            action, stderr
        )));
    }

    Ok(())
}

/// Start the web-server service using docker-compose
pub fn start_service(project_config: &ProjectConfig) -> Result<()> {
    let compose_path = get_compose_path(project_config);

    info!(
        "Starting service '{}' via docker-compose...",
        project_config.service_name
    );

    // Try new syntax: docker compose
    let output = Command::new("docker")
        .args([
            "compose",
            "-f",
            compose_path.to_str().unwrap_or_default(),
            "up",
            "-d",
            &project_config.service_name,
        ])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            info!("  Service started");
            return Ok(());
        }
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            // If new syntax fails, try old syntax
            if stderr.contains("is not a docker command")
                || stderr.contains("unknown command")
            {
                let result = try_docker_compose_legacy_start(&compose_path, &project_config.service_name);
                if result.is_ok() {
                    info!("  Service started");
                }
                return result;
            }
            return Err(DeployError::Docker(format!(
                "Failed to start service: {}",
                stderr
            )));
        }
        Err(_) => {
            // docker command not found, try docker-compose
            let result = try_docker_compose_legacy_start(&compose_path, &project_config.service_name);
            if result.is_ok() {
                info!("  Service started");
            }
            return result;
        }
    }
}

/// Start service using legacy docker-compose command
fn try_docker_compose_legacy_start(compose_path: &Path, service_name: &str) -> Result<()> {
    let output = Command::new("docker-compose")
        .args(["-f", compose_path.to_str().unwrap_or_default(), "up", "-d", service_name])
        .output()
        .map_err(|e| DeployError::Docker(format!("Failed to run docker-compose: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Docker(format!(
            "docker-compose up failed: {}",
            stderr
        )));
    }

    Ok(())
}

/// Check if docker is available
pub fn check_docker_available() -> Result<()> {
    info!("Checking Docker availability...");

    let output = Command::new("docker")
        .args(["info"])
        .output()
        .map_err(|e| DeployError::Docker(format!("Docker not available: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Docker(format!(
            "Docker not running or accessible: {}",
            stderr
        )));
    }

    info!("  Docker: OK");
    Ok(())
}
