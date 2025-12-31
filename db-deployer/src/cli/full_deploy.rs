//! Full deployment command implementation
//!
//! This command executes the complete deployment workflow:
//! 1. Check Git permissions
//! 2. Update project repository
//! 3. Sync SQL files
//! 4. Stop service
//! 5. Deploy/upgrade database
//! 6. Update docker-compose.yml
//! 7. Start service

use std::path::Path;

use clap::Args;
use console::style;
use dialoguer::Confirm;
use tracing::info;

use crate::config::Config;
use crate::deploy::{self, DetectedMode};
use crate::error::Result;
use crate::project::{self, compose, docker, git, sync};

#[derive(Args)]
pub struct FullDeployArgs {
    /// Force INIT mode (fresh deployment)
    #[arg(long)]
    pub init: bool,

    /// Force UPGRADE mode
    #[arg(long)]
    pub upgrade: bool,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Dry run (don't actually execute)
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn execute(args: FullDeployArgs, config_path: &str) -> anyhow::Result<()> {
    // Load configuration
    let config = crate::config::load(config_path)?;
    let project_config = project::require_project_config(&config)?;

    println!();
    println!("{}", style("============================================================").cyan());
    println!("{}", style("  Full Deployment Workflow").cyan().bold());
    println!("{}", style("============================================================").cyan());
    println!();

    // Show deployment plan
    println!("{}", style("Deployment Plan:").bold());
    println!("  1. Check Git permissions");
    println!("  2. Update project repository (branch: {})", project_config.branch);
    println!("  3. Sync SQL files from project");
    println!("  4. Stop service: {}", project_config.service_name);
    println!("  5. Deploy/upgrade database");
    println!("  6. Update docker-compose.yml ({})", project_config.db_url_env_name);
    println!("  7. Start service: {}", project_config.service_name);
    println!();

    if args.dry_run {
        println!("{}", style("DRY RUN - No changes will be made").yellow().bold());
        println!();
    }

    // Confirmation
    if !args.yes && !args.dry_run {
        println!("{}", style("WARNING: This will:").yellow());
        println!("  - Discard all local changes in the project repository");
        println!("  - Stop the running service (downtime expected)");
        println!("  - Modify docker-compose.yml locally (will NOT be committed)");
        println!();

        let confirmed = Confirm::new()
            .with_prompt("Proceed with full deployment?")
            .default(false)
            .interact()?;

        if !confirmed {
            println!("{}", style("Deployment cancelled").yellow());
            return Ok(());
        }
    }

    if args.dry_run {
        println!("{}", style("Dry run complete. No changes made.").green());
        return Ok(());
    }

    // Execute full deployment workflow
    execute_full_deploy(&config, project_config, args.init, args.upgrade).await?;

    Ok(())
}

async fn execute_full_deploy(
    config: &Config,
    project_config: &crate::config::ProjectConfig,
    force_init: bool,
    force_upgrade: bool,
) -> Result<()> {
    println!();
    println!("{}", style("Step 1: Checking permissions...").bold());

    // Check Docker availability
    docker::check_docker_available()?;

    // Check local git permission
    let current_dir = std::env::current_dir().map_err(|e| {
        crate::error::DeployError::Git(format!("Failed to get current directory: {}", e))
    })?;
    git::check_local_git_permission(&current_dir)?;

    // Check project git permission
    git::check_project_git_permission(project_config)?;

    println!();
    println!("{}", style("Step 2: Updating project repository...").bold());

    // Clone if needed, then update
    git::clone_project_if_needed(project_config)?;
    git::update_project_repo(project_config)?;

    println!();
    println!("{}", style("Step 3: Syncing SQL files...").bold());

    // Sync SQL files from project to deployment tool
    sync::sync_sql_files(config, project_config)?;

    println!();
    println!("{}", style("Step 4: Stopping service...").bold());

    // Get current DB URL for rollback info
    let old_db_url = compose::get_current_db_url(project_config)?;
    if let Some(ref url) = old_db_url {
        info!("Current database URL: {}", mask_password(url));
    }

    // Stop the service
    docker::stop_service(project_config)?;

    println!();
    println!("{}", style("Step 5: Deploying database...").bold());

    // Determine deploy mode
    let records_path = Path::new(&config.paths.records_dir).join("current.json");
    let mode = if force_init {
        DetectedMode::Init
    } else if force_upgrade {
        DetectedMode::Upgrade
    } else if records_path.exists() {
        DetectedMode::Upgrade
    } else {
        DetectedMode::Init
    };

    info!("Deploy mode: {:?}", mode);

    // Execute database deployment
    let new_db_name = match mode {
        DetectedMode::Init => deploy::init_mode::execute_init(config).await?,
        DetectedMode::Upgrade => deploy::upgrade_mode::execute_upgrade(config).await?,
    };

    println!();
    println!("{}", style("Step 6: Updating docker-compose.yml...").bold());

    // Update docker-compose.yml with new database name
    compose::update_db_url(
        project_config,
        &new_db_name,
        &config.database.host,
        config.database.port,
        &config.database.user,
        &config.database.password,
    )?;

    println!();
    println!("{}", style("Step 7: Starting service...").bold());

    // Start the service
    docker::start_service(project_config)?;

    println!();
    println!("{}", style("============================================================").green());
    println!("{}", style("  Full Deployment Successful!").green().bold());
    println!("{}", style("============================================================").green());
    println!();
    println!("  New database: {}", style(&new_db_name).cyan());
    println!("  Service: {} (running)", style(&project_config.service_name).cyan());
    println!();

    if let Some(old_url) = old_db_url {
        println!("{}", style("Rollback info:").yellow());
        println!("  If you need to rollback, update docker-compose.yml:");
        println!("  {}: {}", project_config.db_url_env_name, mask_password(&old_url));
        println!("  Then restart the service:");
        println!("  docker compose -f {} restart {}", 
            docker::get_compose_path(project_config).display(),
            project_config.service_name);
        println!();
    }

    Ok(())
}

/// Mask password in database URL for display
fn mask_password(url: &str) -> String {
    // postgres://user:password@host:port/db -> postgres://user:****@host:port/db
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let prefix = &url[..=colon_pos];
            let suffix = &url[at_pos..];
            return format!("{}****{}", prefix, suffix);
        }
    }
    url.to_string()
}
