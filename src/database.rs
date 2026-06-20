use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};

use crate::error::SeekrResult;
use crate::types::{FileEntry, IndexStats};

/// Manages the SQLite database for file indexing.
pub struct Database {
    conn: Connection,
}

fn row_to_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<FileEntry> {
    Ok(FileEntry {
        id: row.get(0)?,
        path: PathBuf::from(row.get::<_, String>(1)?),
        file_name: row.get(2)?,
        extension: row.get(3)?,
        parent_dir: PathBuf::from(row.get::<_, String>(4)?),
        size: row.get::<_, i64>(5)? as u64,
        modified: row
            .get::<_, Option<String>>(6)?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        accessed: row
            .get::<_, Option<String>>(7)?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        is_hidden: row.get(8)?,
        is_dir: row.get(9)?,
        hash: row.get(10)?,
    })
}

impl Database {
    /// Opens or creates a database at the given path.
    pub fn open(path: &Path) -> SeekrResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    /// Opens an in-memory database for testing.
    pub fn open_memory() -> SeekrResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> SeekrResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                file_name TEXT NOT NULL,
                extension TEXT,
                parent_dir TEXT NOT NULL,
                size INTEGER NOT NULL DEFAULT 0,
                modified TEXT,
                accessed TEXT,
                is_hidden INTEGER NOT NULL DEFAULT 0,
                is_dir INTEGER NOT NULL DEFAULT 0,
                hash TEXT,
                indexed_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
            CREATE INDEX IF NOT EXISTS idx_files_name ON files(file_name);
            CREATE INDEX IF NOT EXISTS idx_files_ext ON files(extension);
            CREATE INDEX IF NOT EXISTS idx_files_parent ON files(parent_dir);
            CREATE INDEX IF NOT EXISTS idx_files_size ON files(size);
            CREATE INDEX IF NOT EXISTS idx_files_modified ON files(modified);
            CREATE INDEX IF NOT EXISTS idx_files_hidden ON files(is_hidden);",
        )?;
        Ok(())
    }

    /// Inserts or replaces a file entry into the database.
    pub fn upsert_file(&self, entry: &FileEntry) -> SeekrResult<i64> {
        self.conn.execute(
            "INSERT OR REPLACE INTO files
                (path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                entry.path.to_string_lossy(),
                entry.file_name,
                entry.extension,
                entry.parent_dir.to_string_lossy(),
                entry.size as i64,
                entry.modified.map(|d| d.to_rfc3339()),
                entry.accessed.map(|d| d.to_rfc3339()),
                entry.is_hidden,
                entry.is_dir,
                entry.hash,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Inserts or replaces multiple file entries in a transaction.
    pub fn upsert_batch(&self, entries: &[FileEntry]) -> SeekrResult<()> {
        let tx = self.conn.unchecked_transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO files
                    (path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            )?;
            for entry in entries {
                stmt.execute(params![
                    entry.path.to_string_lossy(),
                    entry.file_name,
                    entry.extension,
                    entry.parent_dir.to_string_lossy(),
                    entry.size as i64,
                    entry.modified.map(|d| d.to_rfc3339()),
                    entry.accessed.map(|d| d.to_rfc3339()),
                    entry.is_hidden,
                    entry.is_dir,
                    entry.hash,
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Removes a file entry by path.
    pub fn remove_file(&self, path: &Path) -> SeekrResult<bool> {
        let rows = self.conn.execute(
            "DELETE FROM files WHERE path = ?1",
            params![path.to_string_lossy()],
        )?;
        Ok(rows > 0)
    }

    /// Retrieves a file entry by path.
    pub fn get_file(&self, path: &Path) -> SeekrResult<Option<FileEntry>> {
        let result = self
            .conn
            .query_row(
                "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
                FROM files WHERE path = ?1",
                params![path.to_string_lossy()],
                row_to_entry,
            )
            .optional()?;
        Ok(result)
    }

    /// Searches files by name pattern using SQL LIKE.
    pub fn search_by_name(
        &self,
        pattern: &str,
        case_sensitive: bool,
        limit: i64,
        offset: i64,
    ) -> SeekrResult<Vec<FileEntry>> {
        let like_pattern = format!("%{}%", pattern);
        let query = if case_sensitive {
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE file_name LIKE ?1 ESCAPE '\\'
            ORDER BY file_name LIMIT ?2 OFFSET ?3"
        } else {
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE file_name LIKE ?1 COLLATE NOCASE ESCAPE '\\'
            ORDER BY file_name LIMIT ?2 OFFSET ?3"
        };

        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map(params![like_pattern, limit, offset], row_to_entry)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Searches files by path pattern.
    pub fn search_by_path(
        &self,
        pattern: &str,
        case_sensitive: bool,
        limit: i64,
        offset: i64,
    ) -> SeekrResult<Vec<FileEntry>> {
        let like_pattern = format!("%{}%", pattern);
        let query = if case_sensitive {
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE path LIKE ?1 ESCAPE '\\'
            ORDER BY path LIMIT ?2 OFFSET ?3"
        } else {
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE path LIKE ?1 COLLATE NOCASE ESCAPE '\\'
            ORDER BY path LIMIT ?2 OFFSET ?3"
        };

        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map(params![like_pattern, limit, offset], row_to_entry)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Returns all files matching an extension.
    pub fn search_by_extension(
        &self,
        ext: &str,
        limit: i64,
        offset: i64,
    ) -> SeekrResult<Vec<FileEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE extension = ?1
            ORDER BY file_name LIMIT ?2 OFFSET ?3",
        )?;

        let rows = stmt.query_map(params![ext, limit, offset], row_to_entry)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Returns all files within a size range.
    pub fn search_by_size(
        &self,
        min_size: Option<u64>,
        max_size: Option<u64>,
        limit: i64,
        offset: i64,
    ) -> SeekrResult<Vec<FileEntry>> {
        let min = min_size.unwrap_or(0) as i64;
        let max = max_size.unwrap_or(i64::MAX as u64) as i64;

        let mut stmt = self.conn.prepare(
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files WHERE size BETWEEN ?1 AND ?2
            ORDER BY size LIMIT ?3 OFFSET ?4",
        )?;

        let rows = stmt.query_map(params![min, max, limit, offset], row_to_entry)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Returns all files, optionally limited.
    pub fn get_all_files(&self, limit: i64, offset: i64) -> SeekrResult<Vec<FileEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, file_name, extension, parent_dir, size, modified, accessed, is_hidden, is_dir, hash
            FROM files ORDER BY path LIMIT ?1 OFFSET ?2",
        )?;

        let rows = stmt.query_map(params![limit, offset], row_to_entry)?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Returns index statistics.
    pub fn get_stats(&self, root: &Path) -> SeekrResult<IndexStats> {
        let total_files: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM files WHERE is_dir = 0", [], |row| {
                    row.get(0)
                })?;
        let total_dirs: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM files WHERE is_dir = 1", [], |row| {
                    row.get(0)
                })?;
        let total_size: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(size), 0) FROM files WHERE is_dir = 0",
            [],
            |row| row.get(0),
        )?;
        let hidden_files: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files WHERE is_hidden = 1",
            [],
            |row| row.get(0),
        )?;
        let unique_extensions: i64 = self.conn.query_row(
            "SELECT COUNT(DISTINCT extension) FROM files WHERE extension IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        let last_indexed: Option<String> = self
            .conn
            .query_row("SELECT MAX(indexed_at) FROM files", [], |row| row.get(0))
            .optional()?;

        Ok(IndexStats {
            total_files: total_files as u64,
            total_dirs: total_dirs as u64,
            total_size: total_size as u64,
            hidden_files: hidden_files as u64,
            unique_extensions: unique_extensions as u64,
            last_indexed: last_indexed
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            root_path: root.to_path_buf(),
        })
    }

    /// Clears all entries from the database.
    pub fn clear(&self) -> SeekrResult<()> {
        self.conn.execute("DELETE FROM files", [])?;
        Ok(())
    }

    /// Returns the total number of indexed entries.
    pub fn count(&self) -> SeekrResult<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
        Ok(count as u64)
    }
}
