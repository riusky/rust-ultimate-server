mod types;

pub use types::*;

use crate::error::{DeployError, Result};
use std::path::Path;

pub fn load(path: &str) -> Result<Config> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(DeployError::ConfigNotFound(path.display().to_string()));
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)
        .map_err(|e| DeployError::ConfigInvalid(e.to_string()))?;

    config.validate()?;
    Ok(config)
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        // Check keys directory
        if !Path::new(&self.keys.private_key).exists() {
            return Err(DeployError::PrivateKeyNotFound(
                self.keys.private_key.clone(),
            ));
        }
        if !Path::new(&self.keys.public_key).exists() {
            return Err(DeployError::PublicKeyNotFound(
                self.keys.public_key.clone(),
            ));
        }

        Ok(())
    }

    pub fn validate_keys_for_init(&self) -> Result<()> {
        // For init-keys, we don't require keys to exist
        Ok(())
    }
}
