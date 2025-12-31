use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub paths: PathsConfig,
    pub keys: KeysConfig,
    pub git: GitConfig,
    pub deploy: DeployConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name_prefix: String,
    #[serde(default = "default_admin_db")]
    pub admin_db: String,
}

fn default_admin_db() -> String {
    "postgres".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub init_sql_dir: String,
    pub updates_sql_dir: String,
    pub records_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysConfig {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    #[serde(default = "default_true")]
    pub auto_commit: bool,
    #[serde(default = "default_true")]
    pub auto_push: bool,
    #[serde(default = "default_commit_prefix")]
    pub commit_prefix: String,
}

fn default_true() -> bool {
    true
}

fn default_commit_prefix() -> String {
    "chore(db)".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployConfig {
    #[serde(default = "default_retain_versions")]
    pub retain_versions: usize,
    #[serde(default = "default_true")]
    pub require_confirmation: bool,
}

fn default_retain_versions() -> usize {
    3
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "".to_string(),
                name_prefix: "app".to_string(),
                admin_db: "postgres".to_string(),
            },
            paths: PathsConfig {
                init_sql_dir: "sql/init".to_string(),
                updates_sql_dir: "sql/updates".to_string(),
                records_dir: "db-upgrade".to_string(),
            },
            keys: KeysConfig {
                private_key: "keys/private.pem".to_string(),
                public_key: "keys/public.pem".to_string(),
            },
            git: GitConfig {
                auto_commit: true,
                auto_push: true,
                commit_prefix: "chore(db)".to_string(),
            },
            deploy: DeployConfig {
                retain_versions: 3,
                require_confirmation: true,
            },
        }
    }
}
