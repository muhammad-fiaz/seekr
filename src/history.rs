use chrono::{DateTime, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::SeekrResult;

/// A single search history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Unique identifier.
    pub id: Option<i64>,
    /// The search pattern.
    pub pattern: String,
    /// Whether the search was case-sensitive.
    pub case_sensitive: bool,
    /// Whether regex was used.
    pub use_regex: bool,
    /// Whether fuzzy matching was used.
    pub use_fuzzy: bool,
    /// Number of results returned.
    pub result_count: usize,
    /// When the search was performed.
    pub timestamp: DateTime<Utc>,
}

impl Database {
    /// Returns recent search history entries.
    pub fn get_history(&self, limit: usize) -> SeekrResult<Vec<HistoryEntry>> {
        let mut stmt = self.conn().prepare(
            "SELECT id, pattern, case_sensitive, use_regex, use_fuzzy, result_count, timestamp
            FROM search_history ORDER BY timestamp DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            let ts_str: Option<String> = row.get(6)?;
            let timestamp = ts_str
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            Ok(HistoryEntry {
                id: row.get(0)?,
                pattern: row.get(1)?,
                case_sensitive: row.get(2)?,
                use_regex: row.get(3)?,
                use_fuzzy: row.get(4)?,
                result_count: row.get::<_, i64>(5)? as usize,
                timestamp,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }

    /// Records a search in the history.
    pub fn record_search(
        &self,
        pattern: &str,
        case_sensitive: bool,
        use_regex: bool,
        use_fuzzy: bool,
        result_count: usize,
    ) -> SeekrResult<i64> {
        self.conn().execute(
            "INSERT INTO search_history (pattern, case_sensitive, use_regex, use_fuzzy, result_count)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![pattern, case_sensitive, use_regex, use_fuzzy, result_count as i64],
        )?;
        Ok(self.conn().last_insert_rowid())
    }

    /// Clears all search history.
    pub fn clear_history(&self) -> SeekrResult<()> {
        self.conn().execute("DELETE FROM search_history", [])?;
        Ok(())
    }

    /// Returns the total number of history entries.
    pub fn history_count(&self) -> SeekrResult<u64> {
        let count: i64 =
            self.conn()
                .query_row("SELECT COUNT(*) FROM search_history", [], |row| row.get(0))?;
        Ok(count as u64)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Database;

    #[test]
    fn test_record_and_get_history() {
        let db = Database::open_memory().unwrap();
        db.record_search("test", false, false, false, 5).unwrap();
        db.record_search("hello", false, false, false, 10).unwrap();

        let history = db.get_history(10).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].pattern, "hello");
        assert_eq!(history[1].pattern, "test");
    }

    #[test]
    fn test_history_limit() {
        let db = Database::open_memory().unwrap();
        for i in 0..5 {
            db.record_search(&format!("query{}", i), false, false, false, i)
                .unwrap();
        }
        let history = db.get_history(3).unwrap();
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_clear_history() {
        let db = Database::open_memory().unwrap();
        db.record_search("test", false, false, false, 5).unwrap();
        assert_eq!(db.history_count().unwrap(), 1);
        db.clear_history().unwrap();
        assert_eq!(db.history_count().unwrap(), 0);
    }

    #[test]
    fn test_history_count() {
        let db = Database::open_memory().unwrap();
        assert_eq!(db.history_count().unwrap(), 0);
        db.record_search("a", false, false, false, 1).unwrap();
        db.record_search("b", true, false, false, 2).unwrap();
        assert_eq!(db.history_count().unwrap(), 2);
    }

    #[test]
    fn test_record_search_flags() {
        let db = Database::open_memory().unwrap();
        db.record_search("test", true, true, false, 5).unwrap();
        let history = db.get_history(10).unwrap();
        assert_eq!(history.len(), 1);
        assert!(history[0].case_sensitive);
        assert!(history[0].use_regex);
        assert!(!history[0].use_fuzzy);
    }
}
