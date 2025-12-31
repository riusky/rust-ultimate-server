use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeployError {
    // Config errors
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid configuration: {0}")]
    ConfigInvalid(String),

    // Key errors
    #[error("Private key not found: {0}")]
    PrivateKeyNotFound(String),

    #[error("Public key not found: {0}")]
    PublicKeyNotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    // Signature errors
    #[error("Signature verification failed")]
    SignatureInvalid,

    #[error("Signature file not found: {0}")]
    SignatureNotFound(String),

    // Lock errors
    #[error("Deployment is already in progress (lock file exists)")]
    DeploymentLocked,

    #[error("Failed to acquire deployment lock: {0}")]
    LockAcquireFailed(String),

    // SQL errors
    #[error("SQL file modified: {0}")]
    SqlFileModified(String),

    #[error("SQL file deleted: {0}")]
    SqlFileDeleted(String),

    #[error("SQL execution failed: {0}")]
    SqlExecutionFailed(String),

    #[error("Init SQL directory not found or empty: {0}")]
    InitSqlDirEmpty(String),

    // Database errors
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(String),

    #[error("Database already exists: {0}")]
    DatabaseExists(String),

    #[error("Database creation failed: {0}")]
    DatabaseCreationFailed(String),

    #[error("Template copy failed: {0}")]
    TemplateCopyFailed(String),

    // Record errors
    #[error("Record file corrupted: {0}")]
    RecordCorrupted(String),

    #[error("Record file not found")]
    RecordNotFound,

    // Git errors
    #[error("Git operation failed: {0}")]
    GitOperationFailed(String),

    #[error("Git error: {0}")]
    Git(String),

    // Docker errors
    #[error("Docker error: {0}")]
    Docker(String),

    // Config errors (generic)
    #[error("Configuration error: {0}")]
    Config(String),

    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // Other errors
    #[error("User cancelled operation")]
    UserCancelled,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DeployError>;
