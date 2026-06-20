# Quick Start

## 1. Index Your Files

```bash
seekr index .
```

This scans the current directory and builds a searchable index.

## 2. Search for Files

```bash
seekr search "main.rs"
```

## 3. Use Filters

```bash
# Search by extension
seekr search "." --extension rs

# Search by size
seekr search "." --min-size 1024

# Regex search
seekr search '\.toml$' --regex

# Fuzzy search
seekr search "cnfg" --fuzzy
```

## 4. Content Search (Grep)

```bash
seekr grep "fn main"
seekr grep "TODO" --case-sensitive
seekr grep "error" --extension rs
```

## 5. ML & Semantic Search

```bash
# ML-based relevance scoring
seekr ml-search "configuration file"

# Semantic search (TF-IDF similarity)
seekr semantic "error handling"
```

## 6. Search History & Saved Searches

```bash
# View search history
seekr history list

# Save a search
seekr saved save "rust-files" "*.rs" --tags "rust,code"

# Load a saved search
seekr saved load "rust-files"
```

## 7. Export Results

```bash
seekr search "config" --format json --output results.json
seekr search "config" --format csv --output results.csv
```

## 8. Watch for Changes

```bash
seekr watch .
```

## Library Usage

```rust
use seekr::core::SeekrApp;
use seekr::types::SearchQuery;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = SeekrApp::default_config()?;

    // Index
    let stats = app.index(Path::new("."))?;
    println!("Indexed {} files", stats.total_files);

    // Search
    let query = SearchQuery {
        pattern: "main.rs".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;
    for r in &results {
        println!("{}", r.entry.path.display());
    }

    // Content search (grep)
    let content_results = app.content_search("fn main", &Default::default())?;

    // ML-based search
    let ml_results = app.ml_search(&query)?;

    // Semantic search
    let semantic_results = app.semantic_search(&query)?;

    // Search history
    app.record_search("main.rs", false, false, false, results.len())?;
    let history = app.get_history(10)?;

    // Saved searches
    app.save_search("my-search", Some("Find main files"), &query, &["rust".into()])?;

    Ok(())
}
```
