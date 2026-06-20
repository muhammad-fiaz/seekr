use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single indexed file entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileEntry {
    /// Unique identifier (row ID in the database).
    pub id: Option<i64>,
    /// Absolute path to the file.
    pub path: PathBuf,
    /// File name including extension.
    pub file_name: String,
    /// File extension without the leading dot.
    pub extension: Option<String>,
    /// Parent directory path.
    pub parent_dir: PathBuf,
    /// File size in bytes.
    pub size: u64,
    /// Last modified timestamp.
    pub modified: Option<DateTime<Utc>>,
    /// Last accessed timestamp.
    pub accessed: Option<DateTime<Utc>>,
    /// Whether the file is hidden (dot-prefixed or platform-hidden).
    pub is_hidden: bool,
    /// Whether the file is a directory.
    pub is_dir: bool,
    /// SHA-256 hash of the file contents (computed on demand).
    pub hash: Option<String>,
}

/// Represents a search query with all filtering options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// The search pattern (filename, path, or regex).
    pub pattern: String,
    /// The root directory to search in.
    pub root: Option<PathBuf>,
    /// Whether to search recursively.
    pub recursive: bool,
    /// Whether the search is case-sensitive.
    pub case_sensitive: bool,
    /// Whether to use regex matching.
    pub use_regex: bool,
    /// Whether to use fuzzy matching.
    pub use_fuzzy: bool,
    /// Filter by file extension.
    pub extension: Option<String>,
    /// Minimum file size in bytes.
    pub min_size: Option<u64>,
    /// Maximum file size in bytes.
    pub max_size: Option<u64>,
    /// Filter by modification date (after).
    pub modified_after: Option<DateTime<Utc>>,
    /// Filter by modification date (before).
    pub modified_before: Option<DateTime<Utc>>,
    /// Whether to include hidden files.
    pub include_hidden: bool,
    /// Whether to include directories.
    pub include_dirs: bool,
    /// Sort field.
    pub sort_by: SortField,
    /// Sort direction.
    pub sort_order: SortOrder,
    /// Maximum number of results.
    pub limit: Option<usize>,
    /// Offset for pagination.
    pub offset: usize,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            root: None,
            recursive: true,
            case_sensitive: false,
            use_regex: false,
            use_fuzzy: false,
            extension: None,
            min_size: None,
            max_size: None,
            modified_after: None,
            modified_before: None,
            include_hidden: false,
            include_dirs: false,
            sort_by: SortField::Relevance,
            sort_order: SortOrder::Descending,
            limit: None,
            offset: 0,
        }
    }
}

/// The field to sort search results by.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortField {
    /// Sort by relevance score.
    Relevance,
    /// Sort by file name.
    Name,
    /// Sort by file path.
    Path,
    /// Sort by file size.
    Size,
    /// Sort by modification date.
    Modified,
    /// Sort by file extension.
    Extension,
}

/// The direction of sorting.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortOrder {
    /// Ascending order (A-Z, 0-9, oldest first).
    Ascending,
    /// Descending order (Z-A, 9-0, newest first).
    Descending,
}

/// A single search result with scoring information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The matched file entry.
    pub entry: FileEntry,
    /// Relevance score (higher is better).
    pub score: f64,
    /// Matched portions of the path.
    pub matched_indices: Vec<usize>,
}

/// Statistics about the index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Total number of indexed files.
    pub total_files: u64,
    /// Total number of indexed directories.
    pub total_dirs: u64,
    /// Total size of all indexed files in bytes.
    pub total_size: u64,
    /// Number of hidden files.
    pub hidden_files: u64,
    /// Number of unique extensions.
    pub unique_extensions: u64,
    /// When the index was last updated.
    pub last_indexed: Option<DateTime<Utc>>,
    /// The root path of the index.
    pub root_path: PathBuf,
}

/// Configuration for the indexer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    /// Directories to ignore.
    pub ignore_dirs: Vec<String>,
    /// File patterns to ignore.
    pub ignore_patterns: Vec<String>,
    /// Whether to follow symbolic links.
    pub follow_links: bool,
    /// Maximum depth for recursion (None = unlimited).
    pub max_depth: Option<usize>,
    /// Maximum file size to index in bytes (None = unlimited).
    pub max_file_size: Option<u64>,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            ignore_dirs: vec![
                ".git".into(),
                "node_modules".into(),
                ".target".into(),
                "__pycache__".into(),
                ".DS_Store".into(),
                "target".into(),
                ".cache".into(),
                ".venv".into(),
                "venv".into(),
                ".idea".into(),
                ".vscode".into(),
            ],
            ignore_patterns: vec![
                "*.pyc".into(),
                "*.pyo".into(),
                "*.o".into(),
                "*.so".into(),
                "*.dll".into(),
                "*.exe".into(),
                "*.bin".into(),
            ],
            follow_links: false,
            max_depth: None,
            max_file_size: None,
        }
    }
}

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// The default search root directory.
    pub search_root: Option<PathBuf>,
    /// Indexer configuration.
    pub indexer: IndexerConfig,
    /// Whether to enable the cache.
    pub cache_enabled: bool,
    /// Cache time-to-live in seconds.
    pub cache_ttl: u64,
    /// Maximum number of search results by default.
    pub default_limit: usize,
    /// Whether to enable color output.
    pub color: bool,
    /// Database file path.
    pub database_path: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            search_root: None,
            indexer: IndexerConfig::default(),
            cache_enabled: true,
            cache_ttl: 3600,
            default_limit: 50,
            color: true,
            database_path: None,
        }
    }
}

/// Represents a watched path and its configuration.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// The path to watch.
    pub path: PathBuf,
    /// Whether to watch recursively.
    pub recursive: bool,
    /// Debounce interval in milliseconds.
    pub debounce_ms: u64,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            recursive: true,
            debounce_ms: 500,
        }
    }
}

/// The type of filesystem event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileEvent {
    /// A file was created.
    Created(PathBuf),
    /// A file was modified.
    Modified(PathBuf),
    /// A file was deleted.
    Deleted(PathBuf),
    /// A file was renamed or moved.
    Renamed {
        /// The old path.
        from: PathBuf,
        /// The new path.
        to: PathBuf,
    },
}

/// Output format for CLI results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable colored output.
    Pretty,
    /// Machine-readable JSON output.
    Json,
    /// CSV output.
    Csv,
}
