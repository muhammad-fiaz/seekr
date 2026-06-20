//! # Seekr Search Examples
//!
//! Demonstrates different search modes: name, regex, fuzzy, and filtered search.

use seekr::core::SeekrApp;
use seekr::types::{AppConfig, SearchQuery, SortField, SortOrder};
use std::fs;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = setup()?;
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;
    app.index(dir.path())?;

    // 1. Basic filename search
    println!("=== Filename Search ===");
    let query = SearchQuery {
        pattern: "main".into(),
        ..SearchQuery::default()
    };
    for r in &app.search(&query)? {
        println!("  {}", r.entry.path.display());
    }

    // 2. Regex search
    println!("\n=== Regex Search (all .rs files) ===");
    let query = SearchQuery {
        pattern: r"\.rs$".into(),
        use_regex: true,
        ..SearchQuery::default()
    };
    for r in &app.search(&query)? {
        println!("  {}", r.entry.path.display());
    }

    // 3. Fuzzy search
    println!("\n=== Fuzzy Search ('cfg') ===");
    let results = app.fuzzy_search("cfg", 10)?;
    for r in &results {
        println!("  {} (score: {:.0})", r.entry.path.display(), r.score);
    }

    // 4. Extension filter
    println!("\n=== Extension Filter (.toml) ===");
    let query = SearchQuery {
        pattern: ".".into(),
        extension: Some("toml".into()),
        ..SearchQuery::default()
    };
    for r in &app.search(&query)? {
        println!("  {}", r.entry.path.display());
    }

    // 5. Size filter
    println!("\n=== Size Filter (10-100 bytes) ===");
    let query = SearchQuery {
        pattern: ".".into(),
        min_size: Some(10),
        max_size: Some(100),
        ..SearchQuery::default()
    };
    for r in &app.search(&query)? {
        println!("  {} ({} bytes)", r.entry.path.display(), r.entry.size);
    }

    // 6. Sort by name ascending
    println!("\n=== Sorted by Name (Ascending) ===");
    let query = SearchQuery {
        pattern: ".".into(),
        sort_by: SortField::Name,
        sort_order: SortOrder::Ascending,
        ..SearchQuery::default()
    };
    for r in &app.search(&query)? {
        println!("  {}", r.entry.file_name);
    }

    Ok(())
}

fn setup() -> Result<TempDir, Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("main.rs"), "fn main() {}")?;
    fs::write(dir.path().join("lib.rs"), "pub mod utils;")?;
    fs::write(dir.path().join("config.toml"), "[section]\nkey = \"value\"")?;
    fs::write(dir.path().join("readme.md"), "# Hello")?;
    let sub = dir.path().join("src");
    fs::create_dir_all(&sub)?;
    fs::write(sub.join("utils.rs"), "pub fn helper() {}")?;
    Ok(dir)
}
