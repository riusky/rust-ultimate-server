//! Project deployment module
//!
//! Handles full deployment workflow including:
//! - Git operations (permission check, repository update)
//! - Docker operations (stop/start services)
//! - SQL file synchronization
//! - Docker Compose configuration modification

pub mod compose;
pub mod docker;
pub mod git;
pub mod sync;

use crate::config::Config;
use crate::error::{DeployError, Result};

/// Validate project configuration exists
pub fn require_project_config(config: &Config) -> Result<&crate::config::ProjectConfig> {
    config.project.as_ref().ok_or_else(|| {
        DeployError::Config(
            "[project] section is required for full-deploy command. \
             Please add project configuration to your config file."
                .to_string(),
        )
    })
}
