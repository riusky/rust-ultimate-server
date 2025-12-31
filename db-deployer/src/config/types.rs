use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub paths: PathsConfig,
    pub keys: KeysConfig,
    pub git: GitConfig,
    pub deploy: DeployConfig,
    #[serde(default)]
    pub project: Option<ProjectConfig>,
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

/// Project deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Git repository URL of the project
    pub repo_url: String,

    /// Branch to deploy
    #[serde(default = "default_branch")]
    pub branch: String,

    /// Directory where the project is stored/cloned
    pub project_dir: String,

    /// Docker Compose file path (relative to project_dir)
    #[serde(default = "default_compose_file")]
    pub docker_compose_file: String,

    /// SQL source directory in the project (relative to project_dir)
    #[serde(default = "default_sql_source_dir")]
    pub sql_source_dir: String,

    /// Service name in docker-compose to stop/start
    #[serde(default = "default_service_name")]
    pub service_name: String,

    /// Database URL environment variable name in docker-compose
    #[serde(default = "default_db_url_env_name")]
    pub db_url_env_name: String,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_compose_file() -> String {
    "docker-compose.yml".to_string()
}

fn default_sql_source_dir() -> String {
    "sql".to_string()
}

fn default_service_name() -> String {
    "web-server".to_string()
}

fn default_db_url_env_name() -> String {
    "SERVICE_DB_URL".to_string()
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
            project: None,
        }
    }
}
