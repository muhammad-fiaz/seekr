use std::path::Path;
use std::sync::Mutex;

use crate::analytics;
use crate::cache::{IndexCache, MetadataCache, SearchCache};
use crate::config;
use crate::content_search::{ContentSearchConfig, ContentSearchResult};
use crate::database::Database;
use crate::error::{SeekrError, SeekrResult};
use crate::history::HistoryEntry;
use crate::indexer;
use crate::ml::LinearRelevanceModel;
use crate::platform;
use crate::saved_searches::SavedSearch;
use crate::search;
use crate::semantic::SemanticEncoder;
use crate::types::*;

/// The main application facade that coordinates all services.
///
/// `SeekrApp` provides a unified API for indexing, searching, watching,
/// and managing the file search engine. It is the primary entry point
/// for library consumers.
pub struct SeekrApp {
    db: Mutex<Database>,
    config: AppConfig,
    search_cache: SearchCache,
    metadata_cache: MetadataCache,
    index_cache: IndexCache,
    ml_model: LinearRelevanceModel,
    semantic_encoder: Mutex<Option<SemanticEncoder>>,
}

impl SeekrApp {
    /// Creates a new `SeekrApp` with the given configuration.
    pub fn new(config: AppConfig) -> SeekrResult<Self> {
        let db_path = config
            .database_path
            .as_deref()
            .unwrap_or_else(|| Path::new(":memory:"));

        let db = Database::open(db_path)?;
        let search_cache = SearchCache::new(config.cache_ttl, 10_000);
        let metadata_cache = MetadataCache::new(config.cache_ttl, 50_000);
        let index_cache = IndexCache::new(300);

        Ok(Self {
            db: Mutex::new(db),
            config,
            search_cache,
            metadata_cache,
            index_cache,
            ml_model: LinearRelevanceModel::new(),
            semantic_encoder: Mutex::new(None),
        })
    }

    /// Creates a new `SeekrApp` with default configuration.
    pub fn default_config() -> SeekrResult<Self> {
        let config = config::load_config(None)?;
        Self::new(config)
    }

    /// Loads configuration from a specific path.
    pub fn from_config_path(path: &Path) -> SeekrResult<Self> {
        let config = config::load_config(Some(path))?;
        Self::new(config)
    }

    /// Returns a reference to the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Indexes the specified directory.
    pub fn index(&self, root: &Path) -> SeekrResult<IndexStats> {
        self.index_full(root, &self.config.indexer.clone())
    }

    /// Indexes the specified directory with custom indexer configuration.
    pub fn index_full(
        &self,
        root: &Path,
        indexer_config: &IndexerConfig,
    ) -> SeekrResult<IndexStats> {
        if !root.exists() {
            return Err(SeekrError::PathNotFound(root.to_path_buf()));
        }

        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let count = indexer::index_directory(&db, root, indexer_config)?;
        tracing::info!("indexed {} entries from {}", count, root.display());
        drop(db);

        self.index_cache.clear();
        self.search_cache.clear();

        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let stats = db.get_stats(root)?;
        drop(db);

        self.rebuild_semantic_encoder().ok();

        Ok(stats)
    }

    /// Performs an incremental index update.
    pub fn index_incremental(
        &self,
        root: &Path,
        since: chrono::DateTime<chrono::Utc>,
    ) -> SeekrResult<IndexStats> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let count = indexer::index_incremental(&db, root, &self.config.indexer, since)?;
        tracing::info!(
            "incrementally indexed {} entries from {}",
            count,
            root.display()
        );
        drop(db);

        self.index_cache.clear();
        self.search_cache.clear();

        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let stats = db.get_stats(root)?;
        drop(db);

        self.rebuild_semantic_encoder().ok();

        Ok(stats)
    }

    /// Removes stale entries from the index.
    pub fn remove_stale(&self) -> SeekrResult<u64> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let removed = indexer::remove_stale_entries(&db)?;
        drop(db);

        if removed > 0 {
            self.index_cache.clear();
            self.search_cache.clear();
        }
        Ok(removed)
    }

    /// Executes a search query.
    pub fn search(&self, query: &SearchQuery) -> SeekrResult<Vec<SearchResult>> {
        let cache_key = format!(
            "{}|{}|{}|{}|{}",
            query.pattern,
            query.case_sensitive,
            query.use_regex,
            query.use_fuzzy,
            query.include_hidden,
        );

        if self.config.cache_enabled
            && let Some(cached) = self.search_cache.get(&cache_key)
        {
            tracing::debug!("search cache hit for: {}", query.pattern);
            return Ok(cached);
        }

        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let results = search::search(&db, query)?;
        drop(db);

        if self.config.cache_enabled {
            self.search_cache.insert(cache_key, results.clone());
        }

        Ok(results)
    }

    /// Performs a quick fuzzy search.
    pub fn fuzzy_search(&self, pattern: &str, limit: usize) -> SeekrResult<Vec<SearchResult>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        search::fuzzy_search(&db, pattern, limit)
    }

    /// Returns index statistics.
    pub fn stats(&self, root: &Path) -> SeekrResult<IndexStats> {
        let cache_key = format!("stats:{}", root.display());

        if let Some(cached) = self.index_cache.get(&cache_key) {
            return Ok((*cached).clone());
        }

        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        let stats = db.get_stats(root)?;
        drop(db);

        self.index_cache.insert(cache_key, stats.clone());

        Ok(stats)
    }

    /// Opens a file with the system's default application.
    pub fn open_file(&self, path: &Path) -> SeekrResult<()> {
        platform::open_file(path)
    }

    /// Opens the containing directory of a file.
    pub fn open_directory(&self, path: &Path) -> SeekrResult<()> {
        platform::open_containing_directory(path)
    }

    /// Reveals a file in the file manager.
    pub fn reveal_file(&self, path: &Path) -> SeekrResult<()> {
        platform::reveal_file(path)
    }

    /// Clears all caches.
    pub fn clear_caches(&self) {
        self.search_cache.clear();
        self.metadata_cache.clear();
        self.index_cache.clear();
    }

    /// Returns the search cache statistics.
    pub fn search_cache_len(&self) -> u64 {
        self.search_cache.len()
    }

    /// Clears the entire index database.
    pub fn clear_index(&self) -> SeekrResult<()> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Index(format!("lock error: {}", e)))?;
        db.clear()?;
        drop(db);
        self.clear_caches();
        Ok(())
    }

    /// Acquires a lock on the underlying database.
    pub fn database(&self) -> std::sync::MutexGuard<'_, Database> {
        self.db.lock().expect("database lock poisoned")
    }

    /// Performs content search (grep-like) within indexed files.
    pub fn content_search(
        &self,
        pattern: &str,
        config: &ContentSearchConfig,
    ) -> SeekrResult<Vec<ContentSearchResult>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let file_count = db.count()? as i64;
        let entries = db.get_all_files(file_count, 0)?;
        drop(db);
        crate::content_search::content_search(&entries, pattern, config)
    }

    /// Records a search in history.
    pub fn record_search(
        &self,
        pattern: &str,
        case_sensitive: bool,
        use_regex: bool,
        use_fuzzy: bool,
        result_count: usize,
    ) -> SeekrResult<i64> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.record_search(pattern, case_sensitive, use_regex, use_fuzzy, result_count)
    }

    /// Returns recent search history.
    pub fn get_history(&self, limit: usize) -> SeekrResult<Vec<HistoryEntry>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.get_history(limit)
    }

    /// Clears search history.
    pub fn clear_history(&self) -> SeekrResult<()> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.clear_history()
    }

    /// Saves a search query.
    pub fn save_search(
        &self,
        name: &str,
        description: Option<&str>,
        query: &SearchQuery,
        tags: &[String],
    ) -> SeekrResult<i64> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.save_search(name, description, query, tags)
    }

    /// Loads a saved search by name.
    pub fn load_saved_search(&self, name: &str) -> SeekrResult<Option<SavedSearch>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.load_search(name)
    }

    /// Lists all saved searches.
    pub fn list_saved_searches(&self) -> SeekrResult<Vec<SavedSearch>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.list_saved_searches()
    }

    /// Deletes a saved search.
    pub fn delete_saved_search(&self, name: &str) -> SeekrResult<bool> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        db.delete_search(name)
    }

    /// Generates an analytics report for indexed files.
    pub fn analytics_report(&self, root: &Path) -> SeekrResult<analytics::AnalyticsReport> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let file_count = db.count()? as i64;
        let entries = db.get_all_files(file_count, 0)?;
        let stats = db.get_stats(root)?;
        drop(db);

        let collector = analytics::AnalyticsCollector::new();
        Ok(analytics::generate_report(&collector, &entries, &stats))
    }

    /// Performs ML-based relevance scoring.
    pub fn ml_search(&self, query: &SearchQuery) -> SeekrResult<Vec<SearchResult>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let file_count = db.count()? as i64;
        let entries = db.get_all_files(file_count, 0)?;
        drop(db);

        let mut results: Vec<SearchResult> = entries
            .into_iter()
            .filter(|e| !e.is_dir)
            .map(|entry| {
                let score = self.ml_model.score_entry(&entry, query);
                SearchResult {
                    entry,
                    score,
                    matched_indices: vec![],
                }
            })
            .filter(|r| r.score > 0.0)
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(query.limit.unwrap_or(50));
        Ok(results)
    }

    /// Performs semantic search using TF-IDF similarity.
    pub fn semantic_search(&self, query: &SearchQuery) -> SeekrResult<Vec<SearchResult>> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let file_count = db.count()? as i64;
        let entries = db.get_all_files(file_count, 0)?;
        drop(db);

        {
            let mut enc = self
                .semantic_encoder
                .lock()
                .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
            if enc.is_none() {
                *enc = Some(SemanticEncoder::build(&entries));
            }
        }

        let enc = self
            .semantic_encoder
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let encoder = enc.as_ref().unwrap();

        let mut results: Vec<SearchResult> = entries
            .into_iter()
            .filter(|e| !e.is_dir)
            .map(|entry| {
                let score = encoder.similarity(query, &entry);
                SearchResult {
                    entry,
                    score,
                    matched_indices: vec![],
                }
            })
            .filter(|r| r.score > 0.0)
            .collect();

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(query.limit.unwrap_or(50));
        Ok(results)
    }

    /// Rebuilds the semantic encoder from current index.
    pub fn rebuild_semantic_encoder(&self) -> SeekrResult<()> {
        let db = self
            .db
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        let file_count = db.count()? as i64;
        let entries = db.get_all_files(file_count, 0)?;
        drop(db);

        let mut enc = self
            .semantic_encoder
            .lock()
            .map_err(|e| SeekrError::Search(format!("lock error: {}", e)))?;
        *enc = Some(SemanticEncoder::build(&entries));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_app() -> SeekrApp {
        let config = AppConfig {
            database_path: None,
            ..AppConfig::default()
        };
        SeekrApp::new(config).unwrap()
    }

    fn setup_test_dir() -> TempDir {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("hello.txt"), "hello world").unwrap();
        fs::write(dir.path().join("world.txt"), "foo bar").unwrap();
        fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("config.toml"), "[section]").unwrap();
        let sub = dir.path().join("subdir");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("nested.txt"), "nested content").unwrap();
        dir
    }

    // -- Basic unit tests --

    #[test]
    fn test_app_creation() {
        let app = setup_app();
        assert!(app.config().cache_enabled);
    }

    #[test]
    fn test_clear_caches() {
        let app = setup_app();
        app.clear_caches();
        assert_eq!(app.search_cache_len(), 0);
    }

    // -- Integration tests --

    #[test]
    fn test_full_index_and_search() {
        let dir = setup_test_dir();
        let app = setup_app();

        let stats = app.index(dir.path()).unwrap();
        assert!(stats.total_files >= 5);
        assert!(stats.total_dirs >= 1);

        let query = SearchQuery {
            pattern: "hello".into(),
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].entry.file_name, "hello.txt");
    }

    #[test]
    fn test_search_case_insensitive() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: "HELLO".into(),
            case_sensitive: false,
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_case_sensitive() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: "HELLO".into(),
            case_sensitive: true,
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_extension_filter() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: ".".into(),
            extension: Some("rs".into()),
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entry.file_name, "main.rs");
    }

    #[test]
    fn test_search_regex() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: r"\.txt".into(),
            use_regex: true,
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        assert!(results.len() >= 2);
    }

    #[test]
    fn test_search_fuzzy() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let results = app.fuzzy_search("hello", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_stats() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let stats = app.stats(dir.path()).unwrap();
        assert!(stats.total_files >= 5);
        assert!(stats.total_size > 0);
    }

    #[test]
    fn test_clear_index() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();
        assert!(app.database().count().unwrap() > 0);

        app.clear_index().unwrap();
        assert_eq!(app.database().count().unwrap(), 0);
    }

    #[test]
    fn test_doctor() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let removed = app.remove_stale().unwrap();
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_index_not_found() {
        let app = setup_app();
        let result = app.index(Path::new("/nonexistent/path/that/does/not/exist"));
        assert!(result.is_err());
    }

    #[test]
    fn test_search_empty_pattern() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: "".into(),
            ..SearchQuery::default()
        };
        let result = app.search(&query);
        assert!(result.is_err());
    }

    #[test]
    fn test_incremental_index() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let since = chrono::Utc::now() - chrono::Duration::hours(1);
        let stats = app.index_incremental(dir.path(), since).unwrap();
        assert!(stats.total_files >= 5);
    }

    #[test]
    fn test_search_size_filter() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: ".".into(),
            min_size: Some(10),
            max_size: Some(100),
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        for r in &results {
            assert!(r.entry.size >= 10 && r.entry.size <= 100);
        }
    }

    #[test]
    fn test_search_sort_by_name() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: ".".to_string(),
            sort_by: SortField::Name,
            sort_order: SortOrder::Ascending,
            ..SearchQuery::default()
        };
        let results = app.search(&query).unwrap();
        if results.len() > 1 {
            assert!(results[0].entry.file_name <= results[1].entry.file_name);
        }
    }

    #[test]
    fn test_ml_search() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: "hello".into(),
            ..SearchQuery::default()
        };
        let results = app.ml_search(&query).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_ml_search_no_match() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query_match = SearchQuery {
            pattern: "hello".into(),
            ..SearchQuery::default()
        };
        let query_no_match = SearchQuery {
            pattern: "zzzznotfound".into(),
            ..SearchQuery::default()
        };
        let results_match = app.ml_search(&query_match).unwrap();
        let results_no_match = app.ml_search(&query_no_match).unwrap();
        // Both may return results (ML scores on all features), but
        // the matching query should have higher scores on average
        if !results_match.is_empty() && !results_no_match.is_empty() {
            let avg_match: f64 =
                results_match.iter().map(|r| r.score).sum::<f64>() / results_match.len() as f64;
            let avg_no_match: f64 = results_no_match.iter().map(|r| r.score).sum::<f64>()
                / results_no_match.len() as f64;
            assert!(
                avg_match > avg_no_match,
                "matching query should score higher on average"
            );
        }
    }

    #[test]
    fn test_ml_search_limit() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: ".".into(),
            limit: Some(2),
            ..SearchQuery::default()
        };
        let results = app.ml_search(&query).unwrap();
        assert!(results.len() <= 2);
    }

    #[test]
    fn test_semantic_search() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let query = SearchQuery {
            pattern: "hello".into(),
            ..SearchQuery::default()
        };
        let results = app.semantic_search(&query).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn test_content_search_basic() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let config = crate::content_search::ContentSearchConfig::default();
        let results = app.content_search("hello", &config).unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.total_matches > 0));
    }

    #[test]
    fn test_content_search_case_sensitive() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let config = crate::content_search::ContentSearchConfig {
            case_sensitive: true,
            ..Default::default()
        };
        let results_upper = app.content_search("HELLO", &config).unwrap();
        let total_upper: usize = results_upper.iter().map(|r| r.total_matches).sum();
        assert_eq!(total_upper, 0);
    }

    #[test]
    fn test_content_search_case_insensitive() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let config = crate::content_search::ContentSearchConfig {
            case_sensitive: false,
            ..Default::default()
        };
        let results = app.content_search("HELLO", &config).unwrap();
        let total: usize = results.iter().map(|r| r.total_matches).sum();
        assert!(total > 0);
    }

    #[test]
    fn test_content_search_with_root() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();

        let sub = dir.path().join("subdir");
        let config_root = crate::content_search::ContentSearchConfig {
            root: Some(sub.clone()),
            ..Default::default()
        };
        let config_all = crate::content_search::ContentSearchConfig::default();
        let results_root = app.content_search("content", &config_root).unwrap();
        let results_all = app.content_search("content", &config_all).unwrap();
        assert!(results_root.len() <= results_all.len());
    }

    #[test]
    fn test_record_and_get_history() {
        let app = setup_app();
        app.record_search("test", false, false, false, 5).unwrap();
        app.record_search("hello", true, true, false, 10).unwrap();
        let history = app.get_history(10).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].pattern, "hello");
        assert_eq!(history[1].pattern, "test");
    }

    #[test]
    fn test_clear_history() {
        let app = setup_app();
        app.record_search("test", false, false, false, 5).unwrap();
        assert!(!app.get_history(10).unwrap().is_empty());
        app.clear_history().unwrap();
        assert!(app.get_history(10).unwrap().is_empty());
    }

    #[test]
    fn test_save_load_delete_search() {
        let app = setup_app();
        let query = SearchQuery {
            pattern: "*.rs".into(),
            ..SearchQuery::default()
        };
        app.save_search("rust", Some("Rust files"), &query, &["rust".into()])
            .unwrap();

        let loaded = app.load_saved_search("rust").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().pattern, "*.rs");

        let list = app.list_saved_searches().unwrap();
        assert_eq!(list.len(), 1);

        assert!(app.delete_saved_search("rust").unwrap());
        assert!(app.load_saved_search("rust").unwrap().is_none());
    }

    #[test]
    fn test_analytics_report() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();
        let report = app.analytics_report(dir.path()).unwrap();
        assert!(report.index_stats.total_files >= 5);
        assert!(!report.file_types.top_extensions.is_empty());
    }

    #[test]
    fn test_rebuild_semantic_encoder() {
        let dir = setup_test_dir();
        let app = setup_app();
        app.index(dir.path()).unwrap();
        app.rebuild_semantic_encoder().unwrap();
        let enc = app.semantic_encoder.lock().unwrap();
        assert!(enc.is_some());
        assert!(enc.as_ref().unwrap().total_documents() > 0);
    }
}
