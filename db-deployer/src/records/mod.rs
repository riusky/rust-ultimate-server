mod current;
mod history;

pub use current::*;
pub use history::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Deployment mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeployMode {
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

/// Database record in current.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRecord {
    pub name: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_db: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_version: Option<String>,
}

/// SQL file record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlFileRecord {
    pub filename: String,
    pub sha256: String,
    pub applied_at: DateTime<Utc>,
}

/// History entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub db_name: String,
    pub created_at: DateTime<Utc>,
}

/// Current deployment record (current.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentRecord {
    pub version: String,
    pub mode: DeployMode,
    pub database: DatabaseRecord,
    #[serde(default)]
    pub init_files: Vec<SqlFileRecord>,
    #[serde(default)]
    pub update_files: Vec<SqlFileRecord>,
    #[serde(default)]
    pub history: Vec<HistoryEntry>,
    #[serde(default = "default_retain_versions")]
    pub retain_versions: usize,
}

fn default_retain_versions() -> usize {
    3
}

impl Default for CurrentRecord {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            mode: DeployMode::Init,
            database: DatabaseRecord {
                name: String::new(),
                created_at: Utc::now(),
                source_db: None,
                init_version: None,
                upgrade_version: None,
            },
            init_files: Vec::new(),
            update_files: Vec::new(),
            history: Vec::new(),
            retain_versions: 3,
        }
    }
}
