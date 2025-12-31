use anyhow::Result;
use clap::Args;
use console::style;

use crate::deploy::DeployLock;
use crate::records;
use crate::sql::scan_sql_directory;
use std::path::Path;

#[derive(Args)]
pub struct StatusArgs {
    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,
}

pub async fn execute(args: StatusArgs, config_path: &str) -> Result<()> {
    println!("{}", style("=".repeat(60)).cyan());
    println!("{}", style("  Database Deployment Status").cyan().bold());
    println!("{}", style("=".repeat(60)).cyan());
    println!();

    // Load config
    let config = load_config_for_status(config_path)?;

    // Check lock status
    print!("  Deployment lock: ");
    if DeployLock::is_locked(&config) {
        if let Some(info) = DeployLock::get_lock_info(&config) {
            println!(
                "{} (PID: {}, started: {})",
                style("LOCKED").red().bold(),
                info.pid,
                info.started_at.format("%Y-%m-%d %H:%M:%S")
            );
        } else {
            println!("{}", style("LOCKED").red().bold());
        }
    } else {
        println!("{}", style("Available").green());
    }

    // Check keys
    print!("  RSA Keys: ");
    let private_exists = Path::new(&config.keys.private_key).exists();
    let public_exists = Path::new(&config.keys.public_key).exists();
    if private_exists && public_exists {
        println!("{}", style("Present").green());
    } else {
        println!("{}", style("Missing").red());
        if !private_exists {
            println!("    - Private key missing: {}", config.keys.private_key);
        }
        if !public_exists {
            println!("    - Public key missing: {}", config.keys.public_key);
        }
    }

    // Check current record
    println!();
    print!("  Current record: ");
    if records::exists(&config) {
        println!("{}", style("Exists").green());

        // Try to load and verify
        match records::load_verified(&config) {
            Ok(record) => {
                println!("    Signature: {}", style("Valid").green());
                println!("    Mode: {:?}", record.mode);
                println!("    Database: {}", style(&record.database.name).cyan());
                println!(
                    "    Created: {}",
                    record.database.created_at.format("%Y-%m-%d %H:%M:%S")
                );
                println!("    Init files: {}", record.init_files.len());
                println!("    Update files: {}", record.update_files.len());
                println!("    History: {} versions", record.history.len());
                println!(
                    "    Retain versions: {}",
                    record.retain_versions
                );

                if args.verbose {
                    println!();
                    println!("  History:");
                    for entry in &record.history {
                        println!(
                            "    - {} ({})",
                            entry.db_name,
                            entry.created_at.format("%Y-%m-%d %H:%M:%S")
                        );
                    }
                }
            }
            Err(e) => {
                println!("    Signature: {}", style("Invalid").red());
                println!("    Error: {}", e);
            }
        }
    } else {
        println!("{}", style("Not found (INIT mode required)").yellow());
    }

    // Check SQL directories
    println!();
    println!("  SQL Directories:");

    let init_dir = Path::new(&config.paths.init_sql_dir);
    print!("    Init ({}): ", config.paths.init_sql_dir);
    if init_dir.exists() {
        let files = scan_sql_directory(init_dir)?;
        println!("{} files", files.len());
        if args.verbose && !files.is_empty() {
            for f in &files {
                println!("      - {}", f.filename);
            }
        }
    } else {
        println!("{}", style("Not found").red());
    }

    let updates_dir = Path::new(&config.paths.updates_sql_dir);
    print!("    Updates ({}): ", config.paths.updates_sql_dir);
    if updates_dir.exists() {
        let files = scan_sql_directory(updates_dir)?;
        println!("{} files", files.len());
        if args.verbose && !files.is_empty() {
            for f in &files {
                println!("      - {}", f.filename);
            }
        }
    } else {
        println!("{}", style("Not found").yellow());
    }

    println!();
    Ok(())
}

fn load_config_for_status(config_path: &str) -> Result<crate::config::Config> {
    let path = std::path::Path::new(config_path);
    if !path.exists() {
        anyhow::bail!(
            "Configuration file not found: {}\n\
             Create one from the example or run with --config <path>",
            config_path
        );
    }

    let content = std::fs::read_to_string(path)?;
    let config: crate::config::Config = toml::from_str(&content)?;
    Ok(config)
}
