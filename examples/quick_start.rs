//! # Seekr Quick Start
//!
//! Demonstrates basic indexing and searching with Seekr.

use seekr::core::SeekrApp;
use seekr::types::{AppConfig, SearchQuery};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an app with an in-memory database
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;

    // Index the current directory
    let stats = app.index(Path::new("."))?;
    println!(
        "Indexed {} files, {} directories",
        stats.total_files, stats.total_dirs
    );

    // Search for Rust files
    let query = SearchQuery {
        pattern: "main".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;

    for result in &results {
        println!(
            "{} (score: {:.0})",
            result.entry.path.display(),
            result.score
        );
    }

    println!("\nFound {} results", results.len());
    Ok(())
}
