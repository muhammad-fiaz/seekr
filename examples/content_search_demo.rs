//! Demonstrates grep-like content search across files.

use seekr::content_search::{ContentSearchConfig, content_search};
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
        pattern: "*".into(),
        include_dirs: false,
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;
    let entries: Vec<_> = results.iter().map(|r| r.entry.clone()).collect();

    let config = ContentSearchConfig {
        case_sensitive: true,
        use_regex: false,
        ..ContentSearchConfig::default()
    };

    let matches = content_search(&entries, "fn main", &config)?;
    println!("Found {} files with content matches", matches.len());

    for result in &matches {
        for m in &result.matches {
            println!(
                "{}:{}: {}",
                result.entry.path.display(),
                m.line_number,
                m.line_content.trim()
            );
        }
    }

    Ok(())
}
