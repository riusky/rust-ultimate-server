use crate::config::Config;
use crate::error::{DeployError, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

/// Create a connection pool to a specific database
pub async fn connect(config: &Config, database: &str) -> Result<PgPool> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        database
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(|e| DeployError::DatabaseConnectionFailed(e.to_string()))
}

/// Create a connection pool to the admin database (postgres)
pub async fn connect_admin(config: &Config) -> Result<PgPool> {
    connect(config, &config.database.admin_db).await
}

/// Check if a database exists
pub async fn database_exists(pool: &PgPool, db_name: &str) -> Result<bool> {
    let result = sqlx::query("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(db_name)
        .fetch_optional(pool)
        .await
        .map_err(|e| DeployError::DatabaseConnectionFailed(e.to_string()))?;

    Ok(result.is_some())
}

/// Get list of databases with given prefix
pub async fn list_databases(pool: &PgPool, prefix: &str) -> Result<Vec<String>> {
    let pattern = format!("{}%", prefix);
    let rows = sqlx::query("SELECT datname FROM pg_database WHERE datname LIKE $1 ORDER BY datname")
        .bind(&pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| DeployError::DatabaseConnectionFailed(e.to_string()))?;

    let databases: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("datname"))
        .collect();

    Ok(databases)
}

/// Drop a database
pub async fn drop_database(pool: &PgPool, db_name: &str) -> Result<()> {
    // First terminate all connections
    terminate_connections(pool, db_name).await?;

    // Then drop
    let query = format!("DROP DATABASE IF EXISTS \"{}\"", db_name);
    sqlx::query(&query)
        .execute(pool)
        .await
        .map_err(|e| DeployError::Other(format!("Failed to drop database: {}", e)))?;

    Ok(())
}

/// Terminate all connections to a database
pub async fn terminate_connections(pool: &PgPool, db_name: &str) -> Result<()> {
    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()"
    )
    .bind(db_name)
    .execute(pool)
    .await
    .map_err(|e| DeployError::Other(format!("Failed to terminate connections: {}", e)))?;

    Ok(())
}
