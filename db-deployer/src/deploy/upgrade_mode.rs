use crate::config::Config;
use crate::db::{
    connect, connect_admin, create_from_template, drop_database, execute_sql_file,
    generate_db_name,
};
use crate::error::Result;
use crate::git::commit_and_push;
use crate::records::{
    archive_current, get_databases_to_cleanup, load_verified, save,
    DatabaseRecord, DeployMode, HistoryEntry, SqlFileRecord,
};
use crate::sql::{check_validation, get_latest_version, scan_sql_directory, validate_sql_files};
use chrono::Utc;
use console::style;
use std::path::Path;


/// Execute UPGRADE mode deployment
pub async fn execute_upgrade(config: &Config) -> Result<String> {
    println!("{}", style("=".repeat(60)).cyan());
    println!(
        "{}",
        style("  UPGRADE Mode - Database Migration").cyan().bold()
    );
    println!("{}", style("=".repeat(60)).cyan());
    println!();

    // 1. Load and verify current record
    println!("{}", style("Verifying current record...").yellow());
    let mut current_record = load_verified(config)?;
    println!("  {}", style("Signature valid").green());
    println!("  Current database: {}", style(&current_record.database.name).cyan());

    // 2. Scan update SQL files
    let updates_dir = Path::new(&config.paths.updates_sql_dir);
    let current_files = scan_sql_directory(updates_dir)?;

    // 3. Validate SQL files
    println!();
    println!("{}", style("Validating SQL files...").yellow());
    let _all_recorded: Vec<_> = current_record
        .init_files
        .iter()
        .chain(current_record.update_files.iter())
        .cloned()
        .collect();

    // Only validate update files
    let validation = validate_sql_files(&current_files, &current_record.update_files);
    check_validation(&validation)?;

    if validation.new_files.is_empty() {
        println!("  {}", style("No new SQL files to apply").green());
        return Ok(current_record.database.name.clone());
    }

    println!(
        "  {} new SQL files to apply",
        style(validation.new_files.len()).yellow()
    );
    for file in &validation.new_files {
        println!("    - {}", file.filename);
    }

    // 4. Generate new database name
    let new_db_name = generate_db_name(config);
    let source_db = current_record.database.name.clone();

    println!();
    println!("  Source database: {}", style(&source_db).cyan());
    println!("  Target database: {}", style(&new_db_name).green().bold());
    println!();

    // 5. Create database from template
    println!("{}", style("Creating database from template...").yellow());
    let admin_pool = connect_admin(config).await?;
    create_from_template(&admin_pool, &source_db, &new_db_name).await?;
    println!("  {}", style("Database created via TEMPLATE").green());

    // 6. Connect to new database
    let pool = connect(config, &new_db_name).await?;

    // 7. Execute new SQL files
    println!();
    println!("{}", style("Executing new SQL files...").yellow());
    for (i, file) in validation.new_files.iter().enumerate() {
        print!(
            "  [{}/{}] {}... ",
            i + 1,
            validation.new_files.len(),
            file.filename
        );
        match execute_sql_file(&pool, file).await {
            Ok(_) => println!("{}", style("OK").green()),
            Err(e) => {
                println!("{}", style("FAILED").red());
                // Cleanup: drop the failed database
                println!("{}", style("Rolling back...").red());
                let _ = drop_database(&admin_pool, &new_db_name).await;
                return Err(e);
            }
        }
    }

    // 8. Archive current record
    println!();
    println!("{}", style("Archiving current record...").yellow());
    archive_current(config)?;
    println!("  {}", style("Archived").green());

    // 9. Update and save new record
    println!("{}", style("Saving new record...").yellow());

    // Add current db to history
    current_record.history.push(HistoryEntry {
        db_name: source_db.clone(),
        created_at: current_record.database.created_at,
    });

    // Add new SQL files to update_files
    for file in &validation.new_files {
        current_record.update_files.push(SqlFileRecord {
            filename: file.filename.clone(),
            sha256: file.sha256.clone(),
            applied_at: Utc::now(),
        });
    }

    // Update database info
    current_record.mode = DeployMode::Upgrade;
    current_record.database = DatabaseRecord {
        name: new_db_name.clone(),
        created_at: Utc::now(),
        source_db: Some(source_db.clone()),
        init_version: current_record.database.init_version,
        upgrade_version: get_latest_version(
            &validation
                .new_files
                .iter()
                .map(|f| crate::sql::SqlFile {
                    filename: f.filename.clone(),
                    path: f.path.clone(),
                    sha256: f.sha256.clone(),
                })
                .collect::<Vec<_>>(),
        ),
    };

    save(config, &current_record)?;
    println!("  {}", style("Record saved and signed").green());

    // 10. Cleanup old databases
    let to_cleanup = get_databases_to_cleanup(&current_record);
    if !to_cleanup.is_empty() {
        println!();
        println!("{}", style("Cleaning up old databases...").yellow());
        for db in &to_cleanup {
            match drop_database(&admin_pool, db).await {
                Ok(_) => println!("  Dropped: {}", db),
                Err(e) => println!("  Failed to drop {}: {}", db, e),
            }
        }
    }

    // 11. Git commit
    println!();
    println!("{}", style("Committing to git...").yellow());
    let applied_files: Vec<String> = validation
        .new_files
        .iter()
        .map(|f| f.filename.clone())
        .collect();

    match commit_and_push(config, &new_db_name, &applied_files, Some(&source_db)) {
        Ok(_) => println!("  {}", style("Git commit successful").green()),
        Err(e) => println!("  {} {}", style("Git warning:").yellow(), e),
    }

    Ok(new_db_name)
}
