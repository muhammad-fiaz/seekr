//! Demonstrates semantic (TF-IDF) search.

use seekr::core::SeekrApp;
use seekr::types::{AppConfig, SearchQuery};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;

    app.index(Path::new("src"))?;

    let query = SearchQuery {
        pattern: "configuration settings".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;

    println!("Found {} results for semantic query", results.len());
    for result in results.iter().take(10) {
        println!("  {}", result.entry.file_name);
    }

    Ok(())
}
