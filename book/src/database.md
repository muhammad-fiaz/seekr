# Database

## SQLite Backend

Seekr uses SQLite via `rusqlite` for persistent storage.

## Schema

```sql
CREATE TABLE files (
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

CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_name ON files(file_name);
CREATE INDEX idx_files_ext ON files(extension);
CREATE INDEX idx_files_parent ON files(parent_dir);
CREATE INDEX idx_files_size ON files(size);
CREATE INDEX idx_files_modified ON files(modified);
CREATE INDEX idx_files_hidden ON files(is_hidden);
```

## Features

- **Prepared statements** for repeated queries
- **Transaction batching** for bulk inserts
- **Index optimization** for fast lookups
- **WAL mode** for concurrent reads

## Location

- **Linux**: `~/.local/share/seekr/seekr.db`
- **macOS**: `~/Library/Application Support/com.muhammad-fiaz.seekr/seekr.db`
- **Windows**: `C:\Users\<user>\AppData\Roaming\muhammad-fiaz\seekr\seekr.db`

## In-Memory Mode

For testing, Seekr supports in-memory SQLite databases:

```rust
let db = Database::open_memory()?;
```
