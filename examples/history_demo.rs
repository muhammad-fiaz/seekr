//! Demonstrates search history tracking.

use seekr::database::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open_memory()?;

    db.record_search("test_query", false, false, false, 5)?;
    db.record_search("hello_world", true, false, false, 12)?;
    db.record_search("another_search", false, true, false, 3)?;

    let history = db.get_history(10)?;
    println!("Search history ({} entries):", history.len());
    for entry in &history {
        println!(
            "  '{}' - {} results, at {}",
            entry.pattern,
            entry.result_count,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S")
        );
    }

    println!("\nTotal searches: {}", db.history_count()?);
    db.clear_history()?;
    println!("After clear: {}", db.history_count()?);

    Ok(())
}
