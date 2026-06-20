use std::sync::Arc;
use std::time::Duration;

use moka::sync::Cache;

use crate::types::SearchResult;

/// A caching layer for search results and metadata.
pub struct SearchCache {
    inner: Cache<String, Vec<SearchResult>>,
}

impl SearchCache {
    /// Creates a new search cache with the given TTL and capacity.
    pub fn new(ttl_seconds: u64, capacity: u64) -> Self {
        let inner = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(capacity)
            .build();

        Self { inner }
    }

    /// Returns a cached search result for the given query key.
    pub fn get(&self, key: &str) -> Option<Vec<SearchResult>> {
        self.inner.get(key)
    }

    /// Inserts a search result into the cache.
    pub fn insert(&self, key: String, results: Vec<SearchResult>) {
        self.inner.insert(key, results);
    }

    /// Removes a specific entry from the cache.
    pub fn invalidate(&self, key: &str) {
        self.inner.invalidate(key);
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> u64 {
        self.inner.entry_count()
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.entry_count() == 0
    }
}

impl Default for SearchCache {
    fn default() -> Self {
        Self::new(3600, 10_000)
    }
}

/// A metadata cache for file entries.
pub struct MetadataCache {
    inner: Cache<String, crate::types::FileEntry>,
}

impl MetadataCache {
    /// Creates a new metadata cache with the given TTL and capacity.
    pub fn new(ttl_seconds: u64, capacity: u64) -> Self {
        let inner = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(capacity)
            .build();

        Self { inner }
    }

    /// Returns a cached file entry for the given path key.
    pub fn get(&self, key: &str) -> Option<crate::types::FileEntry> {
        self.inner.get(key)
    }

    /// Inserts a file entry into the cache.
    pub fn insert(&self, key: String, entry: crate::types::FileEntry) {
        self.inner.insert(key, entry);
    }

    /// Removes a specific entry from the cache.
    pub fn invalidate(&self, key: &str) {
        self.inner.invalidate(key);
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> u64 {
        self.inner.entry_count()
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.entry_count() == 0
    }
}

impl Default for MetadataCache {
    fn default() -> Self {
        Self::new(1800, 50_000)
    }
}

/// An index cache for storing index statistics.
pub struct IndexCache {
    inner: Cache<String, Arc<crate::types::IndexStats>>,
}

impl IndexCache {
    /// Creates a new index cache with the given TTL.
    pub fn new(ttl_seconds: u64) -> Self {
        let inner = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(100)
            .build();

        Self { inner }
    }

    /// Returns cached index stats for the given key.
    pub fn get(&self, key: &str) -> Option<Arc<crate::types::IndexStats>> {
        self.inner.get(key)
    }

    /// Inserts index stats into the cache.
    pub fn insert(&self, key: String, stats: crate::types::IndexStats) {
        self.inner.insert(key, Arc::new(stats));
    }

    /// Clears the cache.
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }
}

impl Default for IndexCache {
    fn default() -> Self {
        Self::new(300)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileEntry, SearchResult};
    use std::path::PathBuf;

    #[test]
    fn test_search_cache_insert_get() {
        let cache = SearchCache::new(60, 100);
        let results = vec![SearchResult {
            entry: FileEntry {
                id: Some(1),
                path: PathBuf::from("/test/file.txt"),
                file_name: "file.txt".into(),
                extension: Some("txt".into()),
                parent_dir: PathBuf::from("/test"),
                size: 100,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            score: 100.0,
            matched_indices: vec![0, 1, 2],
        }];

        cache.insert("test_query".into(), results);
        let cached = cache.get("test_query");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
    }

    #[test]
    fn test_search_cache_invalidate() {
        let cache = SearchCache::new(600, 100);
        cache.insert("key".into(), vec![]);
        assert!(cache.get("key").is_some());
        cache.invalidate("key");
        assert!(cache.get("key").is_none());
    }

    #[test]
    fn test_metadata_cache() {
        let cache = MetadataCache::new(60, 100);
        let entry = FileEntry {
            id: Some(1),
            path: PathBuf::from("/test/file.txt"),
            file_name: "file.txt".into(),
            extension: Some("txt".into()),
            parent_dir: PathBuf::from("/test"),
            size: 100,
            modified: None,
            accessed: None,
            is_hidden: false,
            is_dir: false,
            hash: None,
        };
        cache.insert("/test/file.txt".into(), entry);
        assert!(cache.get("/test/file.txt").is_some());
    }
}
