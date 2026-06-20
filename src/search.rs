use crate::database::Database;
use crate::error::{SeekrError, SeekrResult};
use crate::types::{FileEntry, SearchQuery, SearchResult, SortField, SortOrder};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use regex::Regex;

/// Computes a relevance score for a file entry against a search query.
fn compute_score(entry: &FileEntry, query: &SearchQuery) -> (f64, Vec<usize>) {
    let pattern = &query.pattern;
    let name = &entry.file_name;
    let path_str = entry.path.to_string_lossy();

    if query.use_fuzzy {
        let matcher = SkimMatcherV2::default();
        if let Some(score) = matcher.fuzzy_match(name, pattern) {
            return (score as f64, vec![]);
        }
        if let Some(score) = matcher.fuzzy_match(&path_str, pattern) {
            return (score as f64 * 0.8, vec![]);
        }
        return (0.0, vec![]);
    }

    let base_score = 100.0;

    if query.case_sensitive {
        if let Some(pos) = name.find(pattern) {
            let indices: Vec<usize> = (pos..pos + pattern.len()).collect();
            let exact_bonus = if name == pattern { 50.0 } else { 0.0 };
            let prefix_bonus = if pos == 0 { 25.0 } else { 0.0 };
            return (base_score + exact_bonus + prefix_bonus, indices);
        }
        if let Some(pos) = path_str.find(pattern) {
            let indices: Vec<usize> = (pos..pos + pattern.len()).collect();
            return (base_score * 0.7, indices);
        }
    } else {
        let name_lower = name.to_lowercase();
        let pat_lower = pattern.to_lowercase();
        if let Some(pos) = name_lower.find(&pat_lower) {
            let indices: Vec<usize> = (pos..pos + pat_lower.len()).collect();
            let exact_bonus = if name_lower == pat_lower { 50.0 } else { 0.0 };
            let prefix_bonus = if pos == 0 { 25.0 } else { 0.0 };
            return (base_score + exact_bonus + prefix_bonus, indices);
        }
        let path_lower = path_str.to_lowercase();
        if let Some(pos) = path_lower.find(&pat_lower) {
            let indices: Vec<usize> = (pos..pos + pat_lower.len()).collect();
            return (base_score * 0.7, indices);
        }
    }

    (0.0, vec![])
}

/// Executes a search query against the database and returns ranked results.
pub fn search(db: &Database, query: &SearchQuery) -> SeekrResult<Vec<SearchResult>> {
    if query.pattern.is_empty() {
        return Err(SeekrError::Search("search pattern cannot be empty".into()));
    }

    let limit = query.limit.unwrap_or(500) as i64;
    let offset = query.offset as i64;

    let mut candidates: Vec<FileEntry> = if query.use_regex {
        search_regex(db, &query.pattern, query.case_sensitive, limit * 3)?
    } else if let Some(ref ext) = query.extension {
        db.search_by_extension(ext, limit * 3, offset)?
    } else {
        let mut results =
            db.search_by_name(&query.pattern, query.case_sensitive, limit * 3, offset)?;
        let path_results =
            db.search_by_path(&query.pattern, query.case_sensitive, limit * 3, offset)?;
        for entry in path_results {
            if !results.iter().any(|e| e.path == entry.path) {
                results.push(entry);
            }
        }
        results
    };

    candidates.retain(|entry| {
        if !query.include_hidden && entry.is_hidden {
            return false;
        }
        if !query.include_dirs && entry.is_dir {
            return false;
        }
        if let Some(min) = query.min_size {
            if entry.size < min {
                return false;
            }
        }
        if let Some(max) = query.max_size {
            if entry.size > max {
                return false;
            }
        }
        if let Some(after) = query.modified_after {
            if let Some(modified) = entry.modified {
                if modified < after {
                    return false;
                }
            }
        }
        if let Some(before) = query.modified_before {
            if let Some(modified) = entry.modified {
                if modified > before {
                    return false;
                }
            }
        }
        true
    });

    let mut results: Vec<SearchResult> = candidates
        .into_iter()
        .map(|entry| {
            let (score, matched_indices) = if query.use_regex || query.use_fuzzy {
                // For regex and fuzzy, the candidates are already matched by the DB/regex layer
                // Give them a base score so they pass the filter
                (100.0, vec![])
            } else {
                compute_score(&entry, query)
            };
            SearchResult {
                entry,
                score,
                matched_indices,
            }
        })
        .filter(|r| r.score > 0.0)
        .collect();

    results.sort_by(|a, b| {
        let ord = match query.sort_by {
            SortField::Relevance => b
                .score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::Name => a.entry.file_name.cmp(&b.entry.file_name),
            SortField::Path => a.entry.path.cmp(&b.entry.path),
            SortField::Size => a.entry.size.cmp(&b.entry.size),
            SortField::Modified => a.entry.modified.cmp(&b.entry.modified),
            SortField::Extension => a.entry.extension.cmp(&b.entry.extension),
        };
        match query.sort_order {
            SortOrder::Ascending => ord,
            SortOrder::Descending => ord.reverse(),
        }
    });

    let limit = query.limit.unwrap_or(50);
    results.truncate(limit);

    Ok(results)
}

/// Searches using regex patterns.
fn search_regex(
    db: &Database,
    pattern: &str,
    case_sensitive: bool,
    limit: i64,
) -> SeekrResult<Vec<FileEntry>> {
    let regex = if case_sensitive {
        Regex::new(pattern)
    } else {
        Regex::new(&format!("(?i){}", pattern))
    }
    .map_err(|e| SeekrError::Search(format!("invalid regex: {}", e)))?;

    let all_files = db.get_all_files(limit, 0)?;
    let matched: Vec<FileEntry> = all_files
        .into_iter()
        .filter(|entry| {
            regex.is_match(&entry.file_name) || regex.is_match(&entry.path.to_string_lossy())
        })
        .collect();

    Ok(matched)
}

/// Performs a fuzzy search across all indexed files.
pub fn fuzzy_search(db: &Database, pattern: &str, limit: usize) -> SeekrResult<Vec<SearchResult>> {
    let query = SearchQuery {
        pattern: pattern.to_string(),
        use_fuzzy: true,
        limit: Some(limit),
        ..SearchQuery::default()
    };
    search(db, &query)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;
    use chrono::Utc;

    fn setup_db() -> Database {
        let db = Database::open_memory().unwrap();
        let entries = vec![
            FileEntry {
                id: None,
                path: "/home/user/Documents/report.pdf".into(),
                file_name: "report.pdf".into(),
                extension: Some("pdf".into()),
                parent_dir: "/home/user/Documents".into(),
                size: 1024,
                modified: Some(Utc::now()),
                accessed: Some(Utc::now()),
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            FileEntry {
                id: None,
                path: "/home/user/src/main.rs".into(),
                file_name: "main.rs".into(),
                extension: Some("rs".into()),
                parent_dir: "/home/user/src".into(),
                size: 2048,
                modified: Some(Utc::now()),
                accessed: Some(Utc::now()),
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            FileEntry {
                id: None,
                path: "/home/user/.hidden".into(),
                file_name: ".hidden".into(),
                extension: None,
                parent_dir: "/home/user".into(),
                size: 100,
                modified: Some(Utc::now()),
                accessed: Some(Utc::now()),
                is_hidden: true,
                is_dir: false,
                hash: None,
            },
        ];
        db.upsert_batch(&entries).unwrap();
        db
    }

    #[test]
    fn test_search_by_name() {
        let db = setup_db();
        let query = SearchQuery {
            pattern: "report".into(),
            ..SearchQuery::default()
        };
        let results = search(&db, &query).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].entry.file_name, "report.pdf");
    }

    #[test]
    fn test_search_excludes_hidden() {
        let db = setup_db();
        let query = SearchQuery {
            pattern: "hidden".into(),
            include_hidden: false,
            ..SearchQuery::default()
        };
        let results = search(&db, &query).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_includes_hidden() {
        let db = setup_db();
        let query = SearchQuery {
            pattern: "hidden".into(),
            include_hidden: true,
            ..SearchQuery::default()
        };
        let results = search(&db, &query).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_regex() {
        let db = setup_db();

        // First verify the DB has the files
        let all = db.get_all_files(100, 0).unwrap();
        assert_eq!(all.len(), 3, "DB should have 3 files, got {}", all.len());

        let query = SearchQuery {
            pattern: r"\.rs".into(),
            use_regex: true,
            ..SearchQuery::default()
        };
        let results = search(&db, &query).unwrap();
        assert!(
            !results.is_empty(),
            "Regex search for '.rs' should find at least one result"
        );
        assert_eq!(results[0].entry.file_name, "main.rs");
    }

    #[test]
    fn test_search_empty_pattern() {
        let db = setup_db();
        let query = SearchQuery {
            pattern: "".into(),
            ..SearchQuery::default()
        };
        let result = search(&db, &query);
        assert!(result.is_err());
    }
}
