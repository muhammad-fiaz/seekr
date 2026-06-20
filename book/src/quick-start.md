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

## 4. Export Results

```bash
seekr search "config" --format json --output results.json
seekr search "config" --format csv --output results.csv
```

## 5. Watch for Changes

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

    Ok(())
}
```
