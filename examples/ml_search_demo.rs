//! Demonstrates ML-powered relevance search.

use seekr::core::SeekrApp;
use seekr::ml::LinearRelevanceModel;
use seekr::types::{AppConfig, SearchQuery};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })?;

    app.index(Path::new("src"))?;

    let query = SearchQuery {
        pattern: "file".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;

    let model = LinearRelevanceModel::new();
    let mut scored: Vec<_> = results
        .iter()
        .map(|r| {
            let score = model.score_entry(&r.entry, &query);
            (score, &r.entry)
        })
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    println!("Top 5 ML-scored results:");
    for (score, entry) in scored.iter().take(5) {
        println!("  [{:.1}] {}", score, entry.file_name);
    }

    Ok(())
}
