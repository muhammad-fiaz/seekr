//! Demonstrates saved search functionality.

use seekr::database::Database;
use seekr::types::SearchQuery;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open_memory()?;

    let query = SearchQuery {
        pattern: "*.rs".into(),
        case_sensitive: false,
        ..SearchQuery::default()
    };

    db.save_search(
        "rust-files",
        Some("Find all Rust source files"),
        &query,
        &["rust".into(), "source".into()],
    )?;
    println!("Saved search 'rust-files'");

    if let Some(loaded) = db.load_search("rust-files")? {
        println!("Loaded: {} - pattern: {}", loaded.name, loaded.pattern);
    }

    let all = db.list_saved_searches()?;
    println!("Total saved searches: {}", all.len());

    db.touch_saved_search("rust-files")?;
    if let Some(s) = db.load_search("rust-files")? {
        println!("Use count: {}", s.use_count);
    }

    Ok(())
}
