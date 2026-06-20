use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::database::Database;
use crate::error::SeekrResult;
use crate::types::{FileEntry, IndexerConfig};

/// Checks if a file or directory name should be ignored based on patterns.
fn should_ignore_name(name: &str, ignore_patterns: &[String]) -> bool {
    for pattern in ignore_patterns {
        if let Some(suffix) = pattern.strip_prefix('*') {
            if name.ends_with(suffix) {
                return true;
            }
        } else if name == pattern {
            return true;
        }
    }
    false
}

/// Converts a `SystemTime` to `DateTime<Utc>`.
fn system_time_to_datetime(time: SystemTime) -> DateTime<Utc> {
    DateTime::from(time)
}

/// Builds a `FileEntry` from a `walkdir::DirEntry`.
fn build_entry(entry: &walkdir::DirEntry, config: &IndexerConfig) -> Option<FileEntry> {
    let metadata = entry.metadata().ok()?;
    let path = entry.path().to_path_buf();
    let file_name = entry.file_name().to_string_lossy().to_string();

    if should_ignore_name(&file_name, &config.ignore_patterns) {
        return None;
    }

    let is_hidden = file_name.starts_with('.');
    let is_dir = metadata.is_dir();

    let extension = if is_dir {
        None
    } else {
        path.extension().map(|e| e.to_string_lossy().to_string())
    };

    let parent_dir = path.parent().unwrap_or(Path::new("")).to_path_buf();

    let modified = metadata.modified().ok().map(system_time_to_datetime);

    let accessed = metadata.accessed().ok().map(system_time_to_datetime);

    Some(FileEntry {
        id: None,
        path,
        file_name,
        extension,
        parent_dir,
        size: metadata.len(),
        modified,
        accessed,
        is_hidden,
        is_dir,
        hash: None,
    })
}

/// Indexes a directory tree and stores results in the database.
///
/// This function walks the directory tree in parallel using `rayon`, extracts
/// metadata for each file, and stores the results in the provided database.
pub fn index_directory(db: &Database, root: &Path, config: &IndexerConfig) -> SeekrResult<u64> {
    if !root.exists() {
        return Err(crate::error::SeekrError::PathNotFound(root.to_path_buf()));
    }

    let mut walker = WalkDir::new(root).follow_links(config.follow_links);

    if let Some(depth) = config.max_depth {
        walker = walker.max_depth(depth);
    }

    let entries: Vec<FileEntry> = walker
        .into_iter()
        .par_bridge()
        .filter_map(|e| {
            let entry = e.ok()?;
            let fe = build_entry(&entry, config)?;
            if let Some(max_size) = config.max_file_size {
                if !fe.is_dir && fe.size > max_size {
                    return None;
                }
            }
            Some(fe)
        })
        .collect();

    let count = entries.len() as u64;
    db.upsert_batch(&entries)?;
    Ok(count)
}

/// Performs an incremental update of the index.
///
/// Only re-indexes files that have been modified since the given timestamp.
pub fn index_incremental(
    db: &Database,
    root: &Path,
    config: &IndexerConfig,
    since: DateTime<Utc>,
) -> SeekrResult<u64> {
    if !root.exists() {
        return Err(crate::error::SeekrError::PathNotFound(root.to_path_buf()));
    }

    let mut walker = WalkDir::new(root).follow_links(config.follow_links);

    if let Some(depth) = config.max_depth {
        walker = walker.max_depth(depth);
    }

    let since_system: SystemTime = since.into();

    let entries: Vec<FileEntry> = walker
        .into_iter()
        .par_bridge()
        .filter_map(|e| {
            let entry = e.ok()?;
            let metadata = entry.metadata().ok()?;
            let modified = metadata.modified().ok()?;
            if modified < since_system {
                return None;
            }
            build_entry(&entry, config)
        })
        .collect();

    let count = entries.len() as u64;
    if !entries.is_empty() {
        db.upsert_batch(&entries)?;
    }
    Ok(count)
}

/// Removes stale entries from the index for files that no longer exist.
pub fn remove_stale_entries(db: &Database) -> SeekrResult<u64> {
    let files = db.get_all_files(100_000, 0)?;
    let mut removed = 0u64;

    for entry in &files {
        if !entry.path.exists() {
            db.remove_file(&entry.path)?;
            removed += 1;
        }
    }

    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    #[test]
    fn test_index_directory() {
        let db = Database::open_memory().unwrap();
        let root = std::env::temp_dir().join("seekr_test_index");
        let _ = std::fs::create_dir_all(&root);
        let _ = std::fs::write(root.join("test.txt"), "hello world");

        let config = IndexerConfig::default();
        let count = index_directory(&db, &root, &config).unwrap();
        assert!(count >= 1);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn test_should_ignore_name() {
        let patterns = vec!["*.pyc".into(), ".git".into(), "target".into()];
        assert!(should_ignore_name("module.pyc", &patterns));
        assert!(should_ignore_name(".git", &patterns));
        assert!(should_ignore_name("target", &patterns));
        assert!(!should_ignore_name("main.rs", &patterns));
    }
}
