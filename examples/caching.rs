//! # Seekr Cache Examples
//!
//! Demonstrates the caching layer for search results and metadata.

use seekr::cache::{IndexCache, MetadataCache, SearchCache};
use seekr::types::{FileEntry, SearchResult};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Search Cache
    println!("=== Search Cache ===");
    let cache = SearchCache::new(300, 1000);
    let results = vec![make_result("test.txt")];
    cache.insert("query:test".into(), results.clone());

    if let Some(cached) = cache.get("query:test") {
        println!("Cache hit: {} results", cached.len());
    }
    println!("Cache len: {}", cache.len());

    cache.invalidate("query:test");
    println!("After invalidation: {:?}", cache.get("query:test"));

    // 2. Metadata Cache
    println!("\n=== Metadata Cache ===");
    let meta_cache = MetadataCache::new(300, 1000);
    let entry = make_entry("config.toml");
    meta_cache.insert("/path/to/config.toml".into(), entry);

    if let Some(cached) = meta_cache.get("/path/to/config.toml") {
        println!("Cached entry: {}", cached.file_name);
    }
    println!("Cache len: {}", meta_cache.len());

    // 3. Index Cache
    println!("\n=== Index Cache ===");
    let index_cache = IndexCache::new(300);
    let stats = seekr::types::IndexStats {
        total_files: 42,
        total_dirs: 7,
        total_size: 1_048_576,
        hidden_files: 3,
        unique_extensions: 5,
        last_indexed: None,
        root_path: PathBuf::from("/project"),
    };
    index_cache.insert("stats:/project".into(), stats);

    if let Some(cached) = index_cache.get("stats:/project") {
        println!(
            "Cached stats: {} files, {} dirs",
            cached.total_files, cached.total_dirs
        );
    }

    // 4. Default configurations
    println!("\n=== Default Configs ===");
    let default_search = SearchCache::default();
    println!("Default search cache empty: {}", default_search.is_empty());

    let default_meta = MetadataCache::default();
    println!("Default metadata cache empty: {}", default_meta.is_empty());

    Ok(())
}

fn make_result(name: &str) -> SearchResult {
    SearchResult {
        entry: make_entry(name),
        score: 100.0,
        matched_indices: vec![],
    }
}

fn make_entry(name: &str) -> FileEntry {
    FileEntry {
        id: Some(1),
        path: PathBuf::from(format!("/test/{}", name)),
        file_name: name.into(),
        extension: name.rsplit('.').next().map(String::from),
        parent_dir: PathBuf::from("/test"),
        size: 1024,
        modified: None,
        accessed: None,
        is_hidden: false,
        is_dir: false,
        hash: None,
    }
}
