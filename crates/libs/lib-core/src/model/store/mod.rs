// region:    --- Modules

pub(in crate::model) mod dbx;

use crate::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio_postgres::NoTls;
use tracing::info;

mod embedded_migrations {
	use refinery::embed_migrations;

	// Path relative to crate root: crates/libs/lib-core/migrations
	embed_migrations!("migrations");
}

// endregion: --- Modules

pub type Db = Pool<Postgres>;

/// Run database migrations using Refinery.
/// This function is called during ModelManager initialization.
pub async fn run_db_migrations() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let db_url = &core_config().DB_URL;

	info!("{:<12} - Running DB migrations...", "MIGRATION");

	// Use tokio-postgres for migrations (separate from sqlx pool)
	let (mut client, connection) = tokio_postgres::connect(db_url, NoTls).await?;

	// Spawn connection handler
	tokio::spawn(async move {
		if let Err(e) = connection.await {
			tracing::error!("Postgres connection error during migrations: {e}");
		}
	});

	// Run migrations
	let report = embedded_migrations::migrations::runner()
		.run_async(&mut client)
		.await
		.map_err(|e| {
			tracing::error!("{:<12} - Migration failed: {:?}", "MIGRATION", e);
			e
		})?;

	// Log applied migrations
	for migration in report.applied_migrations() {
		info!(
			"{:<12} - Applied: V{} - {}",
			"MIGRATION",
			migration.version(),
			migration.name(),
		);
	}

	if report.applied_migrations().is_empty() {
		info!("{:<12} - No new migrations to apply", "MIGRATION");
	} else {
		info!(
			"{:<12} - {} migrations applied successfully",
			"MIGRATION",
			report.applied_migrations().len()
		);
	}

	Ok(())
}

pub async fn new_db_pool() -> sqlx::Result<Db> {
	// * See NOTE 1) below
	let max_connections = if cfg!(test) { 1 } else { 5 };

	PgPoolOptions::new()
		.max_connections(max_connections)
		.connect(&core_config().DB_URL)
		.await
}