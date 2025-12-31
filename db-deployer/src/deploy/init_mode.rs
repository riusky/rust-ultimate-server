use crate::config::Config;
use crate::db::{connect, connect_admin, create_empty_database, generate_db_name, drop_database};
use crate::error::{DeployError, Result};
use crate::git::commit_and_push;
use crate::records::{
    save, CurrentRecord, DatabaseRecord, DeployMode, SqlFileRecord,
};
use crate::sql::{get_latest_version, scan_sql_directory, SqlFile};
use chrono::Utc;
use console::style;
use std::path::Path;


/// Execute INIT mode deployment
pub async fn execute_init(config: &Config) -> Result<String> {
    println!("{}", style("=".repeat(60)).cyan());
    println!("{}", style("  INIT Mode - Fresh Database Deployment").cyan().bold());
    println!("{}", style("=".repeat(60)).cyan());
    println!();

    // 1. Scan init SQL files
    let init_dir = Path::new(&config.paths.init_sql_dir);
    let init_files = scan_sql_directory(init_dir)?;

    if init_files.is_empty() {
        return Err(DeployError::InitSqlDirEmpty(
            config.paths.init_sql_dir.clone(),
        ));
    }

    println!("  Found {} init SQL files", init_files.len());

    // 2. Scan update SQL files
    let updates_dir = Path::new(&config.paths.updates_sql_dir);
    let update_files = scan_sql_directory(updates_dir)?;
    println!("  Found {} update SQL files", update_files.len());

    // 3. Generate new database name
    let new_db_name = generate_db_name(config);
    println!();
    println!("  New database: {}", style(&new_db_name).green().bold());
    println!();

    // 4. Create empty database
    println!("{}", style("Creating database...").yellow());
    let admin_pool = connect_admin(config).await?;
    create_empty_database(&admin_pool, &new_db_name).await?;
    println!("  {}", style("Database created").green());

    // 5. Connect to new database
    let pool = connect(config, &new_db_name).await?;

    // 6. Execute init SQL files
    println!();
    println!("{}", style("Executing init SQL files...").yellow());
    for (i, file) in init_files.iter().enumerate() {
        print!("  [{}/{}] {}... ", i + 1, init_files.len(), file.filename);
        match crate::db::execute_sql_file(&pool, file).await {
            Ok(_) => println!("{}", style("OK").green()),
            Err(e) => {
                println!("{}", style("FAILED").red());
                // Cleanup: drop the failed database
                let _ = drop_database(&admin_pool, &new_db_name).await;
                return Err(e);
            }
        }
    }

    // 7. Execute update SQL files
    if !update_files.is_empty() {
        println!();
        println!("{}", style("Executing update SQL files...").yellow());
        for (i, file) in update_files.iter().enumerate() {
            print!("  [{}/{}] {}... ", i + 1, update_files.len(), file.filename);
            match crate::db::execute_sql_file(&pool, file).await {
                Ok(_) => println!("{}", style("OK").green()),
                Err(e) => {
                    println!("{}", style("FAILED").red());
                    let _ = drop_database(&admin_pool, &new_db_name).await;
                    return Err(e);
                }
            }
        }
    }

    // 8. Create and save record
    println!();
    println!("{}", style("Saving deployment record...").yellow());

    let init_version = init_files.last().map(|f| f.filename.clone());
    let upgrade_version = get_latest_version(&update_files);

    let record = CurrentRecord {
        version: "1.0".to_string(),
        mode: DeployMode::Init,
        database: DatabaseRecord {
            name: new_db_name.clone(),
            created_at: Utc::now(),
            source_db: None,
            init_version,
            upgrade_version,
        },
        init_files: to_sql_records(&init_files),
        update_files: to_sql_records(&update_files),
        history: Vec::new(),
        retain_versions: config.deploy.retain_versions,
    };

    save(config, &record)?;
    println!("  {}", style("Record saved and signed").green());

    // 9. Git commit
    println!();
    println!("{}", style("Committing to git...").yellow());
    let applied_files: Vec<String> = init_files
        .iter()
        .chain(update_files.iter())
        .map(|f| f.filename.clone())
        .collect();

    match commit_and_push(config, &new_db_name, &applied_files, None) {
        Ok(_) => println!("  {}", style("Git commit successful").green()),
        Err(e) => println!("  {} {}", style("Git warning:").yellow(), e),
    }

    Ok(new_db_name)
}

fn to_sql_records(files: &[SqlFile]) -> Vec<SqlFileRecord> {
    files
        .iter()
        .map(|f| SqlFileRecord {
            filename: f.filename.clone(),
            sha256: f.sha256.clone(),
            applied_at: Utc::now(),
        })
        .collect()
}
