//! Git operations for project deployment
//!
//! Handles:
//! - Git permission validation
//! - Repository update (fetch + reset)

use std::path::Path;
use std::process::Command;

use tracing::{info, warn};

use crate::config::ProjectConfig;
use crate::error::{DeployError, Result};

/// Check if we have Git access to the deployment tool repository
pub fn check_local_git_permission(repo_path: &Path) -> Result<()> {
    info!("Checking local Git repository permission...");

    if !repo_path.join(".git").exists() {
        return Err(DeployError::Git(format!(
            "Not a Git repository: {}",
            repo_path.display()
        )));
    }

    // Try to run git status to verify access
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!(
            "Git access denied for local repository: {}",
            stderr
        )));
    }

    info!("  Local Git permission: OK");
    Ok(())
}

/// Check if we have Git access to the remote project repository
pub fn check_project_git_permission(project_config: &ProjectConfig) -> Result<()> {
    info!("Checking project repository Git permission...");

    let project_path = Path::new(&project_config.project_dir);

    // If project directory doesn't exist, we need to clone
    if !project_path.exists() {
        info!("  Project directory does not exist, will clone on first deploy");
        // Verify we can access the remote repo
        return verify_remote_access(&project_config.repo_url);
    }

    // If it exists, verify it's a git repo
    if !project_path.join(".git").exists() {
        return Err(DeployError::Git(format!(
            "Project directory exists but is not a Git repository: {}",
            project_path.display()
        )));
    }

    // Verify remote access by running git ls-remote
    let output = Command::new("git")
        .args(["ls-remote", "--heads", "origin"])
        .current_dir(project_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git ls-remote: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!(
            "Git access denied for project repository: {}",
            stderr
        )));
    }

    info!("  Project Git permission: OK");
    Ok(())
}

/// Verify we can access a remote repository
fn verify_remote_access(repo_url: &str) -> Result<()> {
    info!("  Verifying remote repository access: {}", repo_url);

    let output = Command::new("git")
        .args(["ls-remote", "--heads", repo_url])
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git ls-remote: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!(
            "Cannot access remote repository {}: {}",
            repo_url, stderr
        )));
    }

    info!("  Remote access: OK");
    Ok(())
}

/// Clone the project repository if it doesn't exist
pub fn clone_project_if_needed(project_config: &ProjectConfig) -> Result<()> {
    let project_path = Path::new(&project_config.project_dir);

    if project_path.join(".git").exists() {
        info!("Project repository already exists, skipping clone");
        return Ok(());
    }

    info!(
        "Cloning project repository to {}...",
        project_config.project_dir
    );

    // Create parent directory if needed
    if let Some(parent) = project_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DeployError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create directory {}: {}", parent.display(), e),
                ))
            })?;
        }
    }

    let output = Command::new("git")
        .args([
            "clone",
            "--branch",
            &project_config.branch,
            &project_config.repo_url,
            &project_config.project_dir,
        ])
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git clone: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!("Git clone failed: {}", stderr)));
    }

    info!("  Clone completed");
    Ok(())
}

/// Update project repository: discard all changes and pull latest
pub fn update_project_repo(project_config: &ProjectConfig) -> Result<()> {
    let project_path = Path::new(&project_config.project_dir);

    info!("Updating project repository...");

    // Step 1: Fetch all branches
    info!("  Fetching from origin...");
    let output = Command::new("git")
        .args(["fetch", "origin"])
        .current_dir(project_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git fetch: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!("Git fetch failed: {}", stderr)));
    }

    // Step 2: Clean any untracked files
    info!("  Cleaning untracked files...");
    let output = Command::new("git")
        .args(["clean", "-fd"])
        .current_dir(project_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git clean: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Git clean warning: {}", stderr);
    }

    // Step 3: Reset hard to origin/branch
    let reset_target = format!("origin/{}", project_config.branch);
    info!("  Resetting to {}...", reset_target);
    let output = Command::new("git")
        .args(["reset", "--hard", &reset_target])
        .current_dir(project_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to run git reset: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DeployError::Git(format!("Git reset failed: {}", stderr)));
    }

    // Show current commit
    let output = Command::new("git")
        .args(["log", "-1", "--oneline"])
        .current_dir(project_path)
        .output()
        .map_err(|e| DeployError::Git(format!("Failed to get git log: {}", e)))?;

    if output.status.success() {
        let commit = String::from_utf8_lossy(&output.stdout);
        info!("  Current commit: {}", commit.trim());
    }

    info!("  Repository updated");
    Ok(())
}
