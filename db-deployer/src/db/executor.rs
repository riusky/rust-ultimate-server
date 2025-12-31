use crate::error::{DeployError, Result};
use crate::sql::SqlFile;
use sqlx::PgPool;
use std::fs;
use tracing::{error, info};

/// Execute a single SQL file
pub async fn execute_sql_file(pool: &PgPool, sql_file: &SqlFile) -> Result<()> {
    let content = fs::read_to_string(&sql_file.path)?;

    info!("Executing SQL file: {}", sql_file.filename);

    // Execute the SQL
    sqlx::query(&content)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("SQL execution failed: {}", e);
            DeployError::SqlExecutionFailed(format!("{}: {}", sql_file.filename, e))
        })?;

    Ok(())
}

/// Execute multiple SQL files in order
pub async fn execute_sql_files(pool: &PgPool, sql_files: &[SqlFile]) -> Result<()> {
    for sql_file in sql_files {
        execute_sql_file(pool, sql_file).await?;
    }
    Ok(())
}

/// Execute SQL files with progress callback
pub async fn execute_sql_files_with_progress<F>(
    pool: &PgPool,
    sql_files: &[SqlFile],
    mut on_progress: F,
) -> Result<()>
where
    F: FnMut(usize, usize, &str),
{
    let total = sql_files.len();
    for (i, sql_file) in sql_files.iter().enumerate() {
        on_progress(i + 1, total, &sql_file.filename);
        execute_sql_file(pool, sql_file).await?;
    }
    Ok(())
}
