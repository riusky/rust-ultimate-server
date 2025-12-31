use anyhow::Result;
use clap::Args;
use console::style;
use dialoguer::Confirm;

use crate::deploy::{detect_mode, init_mode, upgrade_mode, DeployLock, DetectedMode};

#[derive(Args)]
pub struct DeployArgs {
    /// Force INIT mode (fresh deployment)
    #[arg(long)]
    pub init: bool,

    /// Force UPGRADE mode
    #[arg(long)]
    pub upgrade: bool,

    /// Dry run - don't actually execute
    #[arg(long)]
    pub dry_run: bool,

    /// Skip confirmation prompts
    #[arg(short, long)]
    pub yes: bool,
}

pub async fn execute(args: DeployArgs, config_path: &str) -> Result<()> {
    // Load config (without key validation for now)
    let config = load_config_for_deploy(config_path)?;

    // Acquire deployment lock
    println!("{}", style("Acquiring deployment lock...").yellow());
    let lock = DeployLock::acquire(&config)?;
    println!("  {}", style("Lock acquired").green());
    println!();

    // Detect or force mode
    let mode = if args.init {
        DetectedMode::Init
    } else if args.upgrade {
        DetectedMode::Upgrade
    } else {
        detect_mode(&config)
    };

    println!(
        "Detected mode: {}",
        match mode {
            DetectedMode::Init => style("INIT").yellow().bold(),
            DetectedMode::Upgrade => style("UPGRADE").cyan().bold(),
        }
    );
    println!();

    // Confirmation
    if config.deploy.require_confirmation && !args.yes && !args.dry_run {
        let confirmed = Confirm::new()
            .with_prompt("Proceed with deployment?")
            .default(false)
            .interact()?;

        if !confirmed {
            println!("{}", style("Deployment cancelled").red());
            lock.release()?;
            return Ok(());
        }
    }

    if args.dry_run {
        println!("{}", style("Dry run - no changes will be made").yellow());
        lock.release()?;
        return Ok(());
    }

    // Execute deployment
    let result = match mode {
        DetectedMode::Init => init_mode::execute_init(&config).await,
        DetectedMode::Upgrade => upgrade_mode::execute_upgrade(&config).await,
    };

    // Release lock
    lock.release()?;

    match result {
        Ok(db_name) => {
            println!();
            println!("{}", style("=".repeat(60)).green());
            println!("{}", style("  Deployment Successful!").green().bold());
            println!("{}", style("=".repeat(60)).green());
            println!();
            println!("  Database: {}", style(&db_name).green().bold());
            println!();
            println!("  Update your application configuration:");
            println!(
                "  DATABASE_URL=postgres://{}:****@{}:{}/{}",
                config.database.user, config.database.host, config.database.port, db_name
            );
            println!();
            Ok(())
        }
        Err(e) => {
            println!();
            println!("{}", style("=".repeat(60)).red());
            println!("{}", style("  Deployment Failed!").red().bold());
            println!("{}", style("=".repeat(60)).red());
            println!();
            println!("  Error: {}", style(&e.to_string()).red());
            println!();
            Err(e.into())
        }
    }
}

fn load_config_for_deploy(config_path: &str) -> Result<crate::config::Config> {
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

    // Validate keys exist
    if !std::path::Path::new(&config.keys.private_key).exists() {
        anyhow::bail!(
            "Private key not found: {}\n\
             Run 'db-deployer init-keys' to generate keys",
            config.keys.private_key
        );
    }

    if !std::path::Path::new(&config.keys.public_key).exists() {
        anyhow::bail!(
            "Public key not found: {}\n\
             Run 'db-deployer init-keys' to generate keys",
            config.keys.public_key
        );
    }

    Ok(config)
}
