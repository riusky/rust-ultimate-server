use crate::config::Config;
use crate::records;

/// Detected deployment mode
#[derive(Debug, Clone, PartialEq)]
pub enum DetectedMode {
    /// No current.json exists - needs initialization
    Init,
    /// current.json exists - upgrade mode
    Upgrade,
}

/// Detect the deployment mode based on current state
pub fn detect_mode(config: &Config) -> DetectedMode {
    if records::exists(config) {
        DetectedMode::Upgrade
    } else {
        DetectedMode::Init
    }
}
