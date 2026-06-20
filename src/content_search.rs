use crate::error::{SeekrError, SeekrResult};
use crate::types::FileEntry;
use rayon::prelude::*;
use regex::Regex;
use std::path::PathBuf;
/// A single content match within a file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentMatch {
    /// The line number (1-indexed).
    pub line_number: usize,
    /// The matched line content.
    pub line_content: String,
    /// Start byte offset of the match within the line.
    pub start_offset: usize,
    /// End byte offset of the match within the line.
    pub end_offset: usize,
}

/// A content search result for a single file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentSearchResult {
    /// The file entry.
    pub entry: FileEntry,
    /// All matches in this file.
    pub matches: Vec<ContentMatch>,
    /// Total number of matches in this file.
    pub total_matches: usize,
}

/// Configuration for content search.
#[derive(Debug, Clone)]
pub struct ContentSearchConfig {
    /// Whether the search is case-sensitive.
    pub case_sensitive: bool,
    /// Whether to use regex matching.
    pub use_regex: bool,
    /// Maximum file size to search (in bytes).
    pub max_file_size: Option<u64>,
    /// Filter by file extension.
    pub extension: Option<String>,
    /// Number of context lines before each match.
    pub context_before: usize,
    /// Number of context lines after each match.
    pub context_after: usize,
    /// Maximum number of results.
    pub limit: Option<usize>,
    /// Restrict search to files under this root path.
    pub root: Option<PathBuf>,
}

impl Default for ContentSearchConfig {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            use_regex: false,
            max_file_size: Some(10 * 1024 * 1024), // 10MB default
            extension: None,
            context_before: 0,
            context_after: 0,
            limit: None,
            root: None,
        }
    }
}

/// Searches file contents for a pattern (grep-like functionality).
pub fn content_search(
    entries: &[FileEntry],
    pattern: &str,
    config: &ContentSearchConfig,
) -> SeekrResult<Vec<ContentSearchResult>> {
    if pattern.is_empty() {
        return Err(SeekrError::Search("search pattern cannot be empty".into()));
    }

    let regex = if config.use_regex {
        let re = if config.case_sensitive {
            Regex::new(pattern)
        } else {
            Regex::new(&format!("(?i){}", pattern))
        }
        .map_err(|e| SeekrError::Search(format!("invalid regex: {}", e)))?;
        Some(re)
    } else {
        None
    };

    let pattern_lower = if config.case_sensitive {
        None
    } else {
        Some(pattern.to_lowercase())
    };

    let candidates: Vec<&FileEntry> = entries
        .iter()
        .filter(|e| {
            if e.is_dir {
                return false;
            }
            if let Some(max_size) = config.max_file_size
                && e.size > max_size
            {
                return false;
            }
            if let Some(ref ext) = config.extension
                && e.extension.as_deref() != Some(ext.as_str())
            {
                return false;
            }
            if let Some(ref root) = config.root {
                let entry_parent = e.parent_dir.as_path();
                if !entry_parent.starts_with(root) {
                    return false;
                }
            }
            true
        })
        .collect();

    let results: Vec<ContentSearchResult> = candidates
        .par_iter()
        .filter_map(|entry| {
            search_file_contents(entry, pattern, config, &regex, pattern_lower.as_deref()).ok()
        })
        .filter(|r| !r.matches.is_empty())
        .collect();

    let mut results = results;
    if let Some(limit) = config.limit {
        results.truncate(limit);
    }

    Ok(results)
}

/// Searches the contents of a single file for a pattern.
fn search_file_contents(
    entry: &FileEntry,
    pattern: &str,
    _config: &ContentSearchConfig,
    regex: &Option<Regex>,
    pattern_lower: Option<&str>,
) -> SeekrResult<ContentSearchResult> {
    let content = std::fs::read_to_string(&entry.path).map_err(|e| {
        SeekrError::Io(std::io::Error::other(format!(
            "failed to read {}: {}",
            entry.path.display(),
            e
        )))
    })?;

    let mut matches = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let mut line_matches = Vec::new();

        if let Some(re) = regex {
            for mat in re.find_iter(line) {
                line_matches.push(ContentMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    start_offset: mat.start(),
                    end_offset: mat.end(),
                });
            }
        } else if let Some(pat_lower) = pattern_lower {
            let line_lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) = line_lower[start..].find(pat_lower) {
                let absolute_pos = start + pos;
                line_matches.push(ContentMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    start_offset: absolute_pos,
                    end_offset: absolute_pos + pattern.len(),
                });
                start = absolute_pos + 1;
            }
        } else {
            let mut start = 0;
            while let Some(pos) = line[start..].find(pattern) {
                let absolute_pos = start + pos;
                line_matches.push(ContentMatch {
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    start_offset: absolute_pos,
                    end_offset: absolute_pos + pattern.len(),
                });
                start = absolute_pos + 1;
            }
        }

        matches.extend(line_matches);
    }

    let total_matches = matches.len();

    Ok(ContentSearchResult {
        entry: entry.clone(),
        matches,
        total_matches,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_files() -> (TempDir, Vec<FileEntry>) {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("hello.txt"), "hello world\nfoo bar").unwrap();
        fs::write(
            dir.path().join("main.rs"),
            "fn main() {\n    println!(\"hello\");\n}",
        )
        .unwrap();
        fs::write(dir.path().join("test.md"), "# Test\nhello world").unwrap();

        let entries: Vec<FileEntry> = vec![
            FileEntry {
                id: Some(1),
                path: dir.path().join("hello.txt"),
                file_name: "hello.txt".into(),
                extension: Some("txt".into()),
                parent_dir: dir.path().to_path_buf(),
                size: 25,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            FileEntry {
                id: Some(2),
                path: dir.path().join("main.rs"),
                file_name: "main.rs".into(),
                extension: Some("rs".into()),
                parent_dir: dir.path().to_path_buf(),
                size: 50,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            FileEntry {
                id: Some(3),
                path: dir.path().join("test.md"),
                file_name: "test.md".into(),
                extension: Some("md".into()),
                parent_dir: dir.path().to_path_buf(),
                size: 20,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
        ];

        (dir, entries)
    }

    #[test]
    fn test_content_search_basic() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig::default();
        let results = content_search(&entries, "hello", &config).unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.total_matches > 0));
    }

    #[test]
    fn test_content_search_empty_pattern() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig::default();
        let result = content_search(&entries, "", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_content_search_case_sensitive() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            case_sensitive: true,
            ..ContentSearchConfig::default()
        };
        let results = content_search(&entries, "Hello", &config).unwrap();
        let total: usize = results.iter().map(|r| r.total_matches).sum();
        assert_eq!(total, 0);
    }

    #[test]
    fn test_content_search_case_insensitive() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            case_sensitive: false,
            ..ContentSearchConfig::default()
        };
        let results = content_search(&entries, "Hello", &config).unwrap();
        let total: usize = results.iter().map(|r| r.total_matches).sum();
        assert!(total > 0);
    }

    #[test]
    fn test_content_search_regex() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            use_regex: true,
            ..ContentSearchConfig::default()
        };
        let results = content_search(&entries, r"fn \w+", &config).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_content_search_extension_filter() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            extension: Some("rs".into()),
            ..ContentSearchConfig::default()
        };
        let results = content_search(&entries, "fn", &config).unwrap();
        assert!(results.len() <= 1);
    }

    #[test]
    fn test_content_search_limit() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            limit: Some(1),
            ..ContentSearchConfig::default()
        };
        let results = content_search(&entries, "hello", &config).unwrap();
        assert!(results.len() <= 1);
    }

    #[test]
    fn test_content_search_no_matches() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig::default();
        let results = content_search(&entries, "zzzznotfound", &config).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_content_search_regex_invalid() {
        let (_dir, entries) = setup_test_files();
        let config = ContentSearchConfig {
            use_regex: true,
            ..ContentSearchConfig::default()
        };
        let result = content_search(&entries, "[invalid", &config);
        assert!(result.is_err());
    }
}
