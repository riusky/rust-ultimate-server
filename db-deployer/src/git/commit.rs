use crate::config::Config;
use crate::error::{DeployError, Result};
use git2::{Repository, Signature};
use std::path::Path;
use tracing::{info, warn};

/// Commit and push changes to git
pub fn commit_and_push(
    config: &Config,
    db_name: &str,
    applied_files: &[String],
    previous_db: Option<&str>,
) -> Result<()> {
    if !config.git.auto_commit {
        info!("Git auto-commit disabled, skipping");
        return Ok(());
    }

    let repo = Repository::discover(".")
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to find repository: {}", e)))?;

    // Add files to index
    let records_dir = Path::new(&config.paths.records_dir);
    add_files_to_index(&repo, records_dir)?;

    // Create commit message
    let mut message = format!("{}: upgrade to {}\n\n", config.git.commit_prefix, db_name);

    if !applied_files.is_empty() {
        message.push_str("Applied SQL:\n");
        for file in applied_files {
            message.push_str(&format!("- {}\n", file));
        }
    }

    if let Some(prev) = previous_db {
        message.push_str(&format!("\nPrevious: {}", prev));
    }

    // Commit
    commit(&repo, &message)?;

    // Push if enabled
    if config.git.auto_push {
        push(&repo)?;
    }

    Ok(())
}

fn add_files_to_index(repo: &Repository, dir: &Path) -> Result<()> {
    let mut index = repo
        .index()
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to get index: {}", e)))?;

    // Add all files in records directory
    index
        .add_all([dir], git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to add files: {}", e)))?;

    index
        .write()
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to write index: {}", e)))?;

    Ok(())
}

fn commit(repo: &Repository, message: &str) -> Result<()> {
    let mut index = repo
        .index()
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to get index: {}", e)))?;

    let tree_id = index
        .write_tree()
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to write tree: {}", e)))?;

    let tree = repo
        .find_tree(tree_id)
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to find tree: {}", e)))?;

    let sig = repo.signature().or_else(|_| {
        Signature::now("db-deployer", "db-deployer@localhost")
            .map_err(|e| DeployError::GitOperationFailed(format!("Failed to create signature: {}", e)))
    })?;

    let parent = repo
        .head()
        .ok()
        .and_then(|h| h.peel_to_commit().ok());

    let parents: Vec<&git2::Commit> = parent.iter().collect();

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to commit: {}", e)))?;

    info!("Git commit created");
    Ok(())
}

fn push(repo: &Repository) -> Result<()> {
    let head = repo
        .head()
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to get HEAD: {}", e)))?;

    let branch_name = head
        .shorthand()
        .ok_or_else(|| DeployError::GitOperationFailed("Failed to get branch name".to_string()))?;

    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| DeployError::GitOperationFailed(format!("Failed to find remote: {}", e)))?;

    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);

    // Try to push, but don't fail the whole operation if it fails
    match remote.push(&[&refspec], None) {
        Ok(_) => {
            info!("Pushed to origin/{}", branch_name);
        }
        Err(e) => {
            warn!("Failed to push (manual push required): {}", e);
        }
    }

    Ok(())
}
