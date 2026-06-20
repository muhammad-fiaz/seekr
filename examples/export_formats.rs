//! # Seekr Export Examples
//!
//! Demonstrates exporting search results in different formats.

use seekr::cli::{export_results, format_size};
use seekr::core::SeekrApp;
use seekr::types::{AppConfig, OutputFormat, SearchQuery};
use std::fs;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = setup()?;
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;
    app.index(dir.path())?;

    let query = SearchQuery {
        pattern: ".".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;

    // 1. JSON export
    let json_path = dir.path().join("results.json");
    export_results(&results, &json_path.to_string_lossy(), &OutputFormat::Json)?;
    let json_content = fs::read_to_string(&json_path)?;
    println!("=== JSON Export ({} bytes) ===", json_content.len());
    println!("{}...", &json_content[..json_content.len().min(200)]);

    // 2. CSV export
    let csv_path = dir.path().join("results.csv");
    export_results(&results, &csv_path.to_string_lossy(), &OutputFormat::Csv)?;
    let csv_content = fs::read_to_string(&csv_path)?;
    println!("\n=== CSV Export ({} bytes) ===", csv_content.len());
    println!("{}", csv_content);

    // 3. Pretty export
    let pretty_path = dir.path().join("results.txt");
    export_results(
        &results,
        &pretty_path.to_string_lossy(),
        &OutputFormat::Pretty,
    )?;
    let pretty_content = fs::read_to_string(&pretty_path)?;
    println!("=== Pretty Export ===");
    println!("{}", pretty_content);

    // 4. Format size utility
    println!("=== Size Formatting ===");
    println!("0 B = {}", format_size(0));
    println!("1024 = {}", format_size(1024));
    println!("1.5 MB = {}", format_size(1_572_864));
    println!("2.5 GB = {}", format_size(2_684_354_560));

    Ok(())
}

fn setup() -> Result<TempDir, Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("data.csv"), "a,b,c")?;
    fs::write(dir.path().join("data.json"), "{}")?;
    fs::write(dir.path().join("data.txt"), "hello")?;
    Ok(dir)
}
