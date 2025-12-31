use crate::error::{DeployError, Result};
use crate::records::SqlFileRecord;
use crate::sql::SqlFile;

/// Validation result for SQL files
#[derive(Debug)]
pub struct ValidationResult {
    pub modified_files: Vec<String>,
    pub deleted_files: Vec<String>,
    pub new_files: Vec<SqlFile>,
    pub is_valid: bool,
}

/// Validate SQL files against recorded hashes
pub fn validate_sql_files(
    current_files: &[SqlFile],
    recorded_files: &[SqlFileRecord],
) -> ValidationResult {
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();
    let mut new_files = Vec::new();

    // Build a map of current files
    let current_map: std::collections::HashMap<&str, &SqlFile> = current_files
        .iter()
        .map(|f| (f.filename.as_str(), f))
        .collect();

    // Check recorded files
    for recorded in recorded_files {
        match current_map.get(recorded.filename.as_str()) {
            Some(current) => {
                // File exists, check if modified
                if current.sha256 != recorded.sha256 {
                    modified_files.push(recorded.filename.clone());
                }
            }
            None => {
                // File was deleted
                deleted_files.push(recorded.filename.clone());
            }
        }
    }

    // Find new files
    let recorded_set: std::collections::HashSet<&str> =
        recorded_files.iter().map(|f| f.filename.as_str()).collect();

    for current in current_files {
        if !recorded_set.contains(current.filename.as_str()) {
            new_files.push(current.clone());
        }
    }

    // Sort new files by filename
    new_files.sort_by(|a, b| a.filename.cmp(&b.filename));

    let is_valid = modified_files.is_empty() && deleted_files.is_empty();

    ValidationResult {
        modified_files,
        deleted_files,
        new_files,
        is_valid,
    }
}

/// Check validation result and return error if invalid
pub fn check_validation(result: &ValidationResult) -> Result<()> {
    if !result.modified_files.is_empty() {
        return Err(DeployError::SqlFileModified(
            result.modified_files.join(", "),
        ));
    }

    if !result.deleted_files.is_empty() {
        return Err(DeployError::SqlFileDeleted(
            result.deleted_files.join(", "),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_no_changes() {
        let current = vec![SqlFile {
            filename: "test.sql".to_string(),
            path: std::path::PathBuf::from("test.sql"),
            sha256: "abc123".to_string(),
        }];

        let recorded = vec![SqlFileRecord {
            filename: "test.sql".to_string(),
            sha256: "abc123".to_string(),
            applied_at: Utc::now(),
        }];

        let result = validate_sql_files(&current, &recorded);
        assert!(result.is_valid);
        assert!(result.new_files.is_empty());
    }

    #[test]
    fn test_validate_modified_file() {
        let current = vec![SqlFile {
            filename: "test.sql".to_string(),
            path: std::path::PathBuf::from("test.sql"),
            sha256: "modified123".to_string(),
        }];

        let recorded = vec![SqlFileRecord {
            filename: "test.sql".to_string(),
            sha256: "abc123".to_string(),
            applied_at: Utc::now(),
        }];

        let result = validate_sql_files(&current, &recorded);
        assert!(!result.is_valid);
        assert_eq!(result.modified_files, vec!["test.sql"]);
    }

    #[test]
    fn test_validate_new_file() {
        let current = vec![
            SqlFile {
                filename: "old.sql".to_string(),
                path: std::path::PathBuf::from("old.sql"),
                sha256: "abc123".to_string(),
            },
            SqlFile {
                filename: "new.sql".to_string(),
                path: std::path::PathBuf::from("new.sql"),
                sha256: "def456".to_string(),
            },
        ];

        let recorded = vec![SqlFileRecord {
            filename: "old.sql".to_string(),
            sha256: "abc123".to_string(),
            applied_at: Utc::now(),
        }];

        let result = validate_sql_files(&current, &recorded);
        assert!(result.is_valid);
        assert_eq!(result.new_files.len(), 1);
        assert_eq!(result.new_files[0].filename, "new.sql");
    }
}
