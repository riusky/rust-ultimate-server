use crate::config::Config;
use crate::db::connection::{database_exists, terminate_connections};
use crate::error::{DeployError, Result};
use chrono::Utc;
use sqlx::PgPool;

/// Generate a new database name with timestamp
pub fn generate_db_name(config: &Config) -> String {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    format!("{}_{}", config.database.name_prefix, timestamp)
}

/// Create a new database using TEMPLATE
pub async fn create_from_template(
    admin_pool: &PgPool,
    source_db: &str,
    target_db: &str,
) -> Result<()> {
    // Check if source exists
    if !database_exists(admin_pool, source_db).await? {
        return Err(DeployError::Other(format!(
            "Source database '{}' does not exist",
            source_db
        )));
    }

    // Check if target already exists
    if database_exists(admin_pool, target_db).await? {
        return Err(DeployError::DatabaseExists(target_db.to_string()));
    }

    // Terminate connections to source
    terminate_connections(admin_pool, source_db).await?;

    // Create new database using TEMPLATE
    let query = format!(
        "CREATE DATABASE \"{}\" TEMPLATE \"{}\"",
        target_db, source_db
    );

    sqlx::query(&query)
        .execute(admin_pool)
        .await
        .map_err(|e| DeployError::TemplateCopyFailed(e.to_string()))?;

    Ok(())
}

/// Create a new empty database
pub async fn create_empty_database(admin_pool: &PgPool, db_name: &str) -> Result<()> {
    // Check if target already exists
    if database_exists(admin_pool, db_name).await? {
        return Err(DeployError::DatabaseExists(db_name.to_string()));
    }

    let query = format!("CREATE DATABASE \"{}\"", db_name);

    sqlx::query(&query)
        .execute(admin_pool)
        .await
        .map_err(|e| DeployError::DatabaseCreationFailed(e.to_string()))?;

    Ok(())
}
