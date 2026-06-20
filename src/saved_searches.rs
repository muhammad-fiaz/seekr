use chrono::{DateTime, Utc};
use rusqlite::{OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::SeekrResult;
use crate::types::SearchQuery;

/// A saved search configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    /// Unique identifier.
    pub id: Option<i64>,
    /// Name for this saved search.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// The search query pattern.
    pub pattern: String,
    /// Whether the search is case-sensitive.
    pub case_sensitive: bool,
    /// Whether regex is used.
    pub use_regex: bool,
    /// Whether fuzzy matching is used.
    pub use_fuzzy: bool,
    /// Filter by extension.
    pub extension: Option<String>,
    /// Tags for organizing searches.
    pub tags: Vec<String>,
    /// When this search was created.
    pub created_at: DateTime<Utc>,
    /// When this search was last used.
    pub last_used: Option<DateTime<Utc>>,
    /// Number of times this search has been used.
    pub use_count: u64,
}

impl Database {
    /// Saves a search query with a name.
    pub fn save_search(
        &self,
        name: &str,
        description: Option<&str>,
        query: &SearchQuery,
        tags: &[String],
    ) -> SeekrResult<i64> {
        let tags_json = serde_json::to_string(tags).unwrap_or_else(|_| "[]".into());
        self.conn().execute(
            "INSERT OR REPLACE INTO saved_searches
            (name, description, pattern, case_sensitive, use_regex, use_fuzzy, extension, tags)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                name,
                description,
                query.pattern,
                query.case_sensitive,
                query.use_regex,
                query.use_fuzzy,
                query.extension,
                tags_json,
            ],
        )?;
        Ok(self.conn().last_insert_rowid())
    }

    /// Loads a saved search by name.
    pub fn load_search(&self, name: &str) -> SeekrResult<Option<SavedSearch>> {
        let result = self
            .conn()
            .query_row(
                "SELECT id, name, description, pattern, case_sensitive, use_regex, use_fuzzy,
                extension, tags, created_at, last_used, use_count
                FROM saved_searches WHERE name = ?1",
                params![name],
                |row| {
                    let tags_str: String = row.get(8)?;
                    let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
                    let created_str: Option<String> = row.get(9)?;
                    let last_used_str: Option<String> = row.get(10)?;
                    Ok(SavedSearch {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        pattern: row.get(3)?,
                        case_sensitive: row.get(4)?,
                        use_regex: row.get(5)?,
                        use_fuzzy: row.get(6)?,
                        extension: row.get(7)?,
                        tags,
                        created_at: created_str
                            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        last_used: last_used_str
                            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                            .map(|dt| dt.with_timezone(&Utc)),
                        use_count: row.get::<_, i64>(11)? as u64,
                    })
                },
            )
            .optional()?;
        Ok(result)
    }

    /// Lists all saved searches.
    pub fn list_saved_searches(&self) -> SeekrResult<Vec<SavedSearch>> {
        let mut stmt = self.conn().prepare(
            "SELECT id, name, description, pattern, case_sensitive, use_regex, use_fuzzy,
            extension, tags, created_at, last_used, use_count
            FROM saved_searches ORDER BY name",
        )?;

        let rows = stmt.query_map([], |row| {
            let tags_str: String = row.get(8)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let created_str: Option<String> = row.get(9)?;
            let last_used_str: Option<String> = row.get(10)?;
            Ok(SavedSearch {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                pattern: row.get(3)?,
                case_sensitive: row.get(4)?,
                use_regex: row.get(5)?,
                use_fuzzy: row.get(6)?,
                extension: row.get(7)?,
                tags,
                created_at: created_str
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                last_used: last_used_str
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                use_count: row.get::<_, i64>(11)? as u64,
            })
        })?;

        let mut searches = Vec::new();
        for row in rows {
            searches.push(row?);
        }
        Ok(searches)
    }

    /// Deletes a saved search by name.
    pub fn delete_search(&self, name: &str) -> SeekrResult<bool> {
        let rows = self
            .conn()
            .execute("DELETE FROM saved_searches WHERE name = ?1", params![name])?;
        Ok(rows > 0)
    }

    /// Updates the last used timestamp and use count for a saved search.
    pub fn touch_saved_search(&self, name: &str) -> SeekrResult<()> {
        self.conn().execute(
            "UPDATE saved_searches SET last_used = strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), use_count = use_count + 1
            WHERE name = ?1",
            params![name],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    fn default_query(pattern: &str) -> SearchQuery {
        SearchQuery {
            pattern: pattern.to_string(),
            ..SearchQuery::default()
        }
    }

    #[test]
    fn test_save_and_load_search() {
        let db = Database::open_memory().unwrap();
        let query = default_query("*.rs");
        db.save_search(
            "rust-files",
            Some("Find Rust files"),
            &query,
            &["rust".into()],
        )
        .unwrap();

        let loaded = db.load_search("rust-files").unwrap();
        assert!(loaded.is_some());
        let s = loaded.unwrap();
        assert_eq!(s.name, "rust-files");
        assert_eq!(s.pattern, "*.rs");
        assert_eq!(s.description, Some("Find Rust files".into()));
        assert_eq!(s.tags, vec!["rust"]);
    }

    #[test]
    fn test_list_saved_searches() {
        let db = Database::open_memory().unwrap();
        db.save_search("a", None, &default_query("a"), &[]).unwrap();
        db.save_search("b", None, &default_query("b"), &[]).unwrap();

        let list = db.list_saved_searches().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_delete_search() {
        let db = Database::open_memory().unwrap();
        db.save_search("to-delete", None, &default_query("x"), &[])
            .unwrap();
        assert!(db.delete_search("to-delete").unwrap());
        assert!(!db.delete_search("nonexistent").unwrap());
        assert!(db.load_search("to-delete").unwrap().is_none());
    }

    #[test]
    fn test_load_nonexistent_search() {
        let db = Database::open_memory().unwrap();
        let result = db.load_search("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_touch_saved_search() {
        let db = Database::open_memory().unwrap();
        db.save_search("test", None, &default_query("q"), &[])
            .unwrap();
        db.touch_saved_search("test").unwrap();
        let s = db.load_search("test").unwrap().unwrap();
        assert!(s.last_used.is_some());
        assert_eq!(s.use_count, 1);
    }

    #[test]
    fn test_save_search_overwrite() {
        let db = Database::open_memory().unwrap();
        db.save_search("test", None, &default_query("old"), &[])
            .unwrap();
        db.save_search("test", None, &default_query("new"), &[])
            .unwrap();
        let s = db.load_search("test").unwrap().unwrap();
        assert_eq!(s.pattern, "new");
    }
}
