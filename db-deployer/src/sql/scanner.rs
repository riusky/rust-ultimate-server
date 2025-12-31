use crate::crypto::hash_file;
use crate::error::Result;
use std::path::Path;
use walkdir::WalkDir;

/// SQL file information
#[derive(Debug, Clone)]
pub struct SqlFile {
    pub filename: String,
    pub path: std::path::PathBuf,
    pub sha256: String,
}

/// Scan a directory for SQL files and compute their hashes
pub fn scan_sql_directory(dir: &Path) -> Result<Vec<SqlFile>> {
    let mut files = Vec::new();

    if !dir.exists() {
        return Ok(files);
    }

    for entry in WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "sql").unwrap_or(false) {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let sha256 = hash_file(path)?;

            files.push(SqlFile {
                filename,
                path: path.to_path_buf(),
                sha256,
            });
        }
    }

    // Sort by filename to ensure consistent ordering
    files.sort_by(|a, b| a.filename.cmp(&b.filename));

    Ok(files)
}

/// Parse version from SQL filename
/// Format: drop-0001-00-00-description.sql -> "0001-00-00"
pub fn parse_version(filename: &str) -> Option<String> {
    let name = filename.strip_prefix("drop-")?;
    let parts: Vec<&str> = name.split('-').collect();
    if parts.len() >= 3 {
        Some(format!("{}-{}-{}", parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

/// Get the latest version from a list of SQL files
pub fn get_latest_version(files: &[SqlFile]) -> Option<String> {
    files
        .iter()
        .filter_map(|f| parse_version(&f.filename))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(
            parse_version("drop-0001-00-00-init.sql"),
            Some("0001-00-00".to_string())
        );
        assert_eq!(
            parse_version("drop-0012-01-03-add-feature.sql"),
            Some("0012-01-03".to_string())
        );
        assert_eq!(parse_version("invalid.sql"), None);
    }
}
