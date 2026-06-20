//! # Seekr Indexing Examples
//!
//! Demonstrates full, incremental, and custom indexing.

use seekr::core::SeekrApp;
use seekr::types::{AppConfig, IndexerConfig};
use std::fs;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = setup()?;
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;

    // 1. Full index
    println!("=== Full Index ===");
    let stats = app.index(dir.path())?;
    println!(
        "Files: {}, Dirs: {}, Size: {} bytes",
        stats.total_files, stats.total_dirs, stats.total_size
    );

    // 2. Custom indexer config
    println!("\n=== Custom Index (max depth 1) ===");
    let config = IndexerConfig {
        max_depth: Some(1),
        ..IndexerConfig::default()
    };
    let stats = app.index_full(dir.path(), &config)?;
    println!("Files: {}, Dirs: {}", stats.total_files, stats.total_dirs);

    // 3. Incremental index
    println!("\n=== Incremental Index ===");
    let since = chrono::Utc::now() - chrono::Duration::hours(1);
    let stats = app.index_incremental(dir.path(), since)?;
    println!("Incrementally indexed {} files", stats.total_files);

    // 4. Clear and re-index
    println!("\n=== Clear and Re-index ===");
    app.clear_index()?;
    println!("Cleared: {} entries", app.database().count()?);
    app.index(dir.path())?;
    println!("Re-indexed: {} entries", app.database().count()?);

    // 5. Remove stale entries
    println!("\n=== Remove Stale ===");
    let removed = app.remove_stale()?;
    println!("Removed {} stale entries", removed);

    Ok(())
}

fn setup() -> Result<TempDir, Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("a.txt"), "file a")?;
    fs::write(dir.path().join("b.txt"), "file b")?;
    fs::write(dir.path().join("c.rs"), "fn c() {}")?;
    let sub = dir.path().join("nested");
    fs::create_dir_all(&sub)?;
    fs::write(sub.join("d.txt"), "file d")?;
    Ok(dir)
}
