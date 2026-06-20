# Database

## SQLite Backend

Seekr uses SQLite via `rusqlite` for persistent storage.

## Schema

### Files Table

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

### Search History Table

```sql
CREATE TABLE search_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL,
    case_sensitive INTEGER NOT NULL DEFAULT 0,
    use_regex INTEGER NOT NULL DEFAULT 0,
    use_fuzzy INTEGER NOT NULL DEFAULT 0,
    result_count INTEGER NOT NULL DEFAULT 0,
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_history_timestamp ON search_history(timestamp);
```

### Saved Searches Table

```sql
CREATE TABLE saved_searches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    pattern TEXT NOT NULL,
    case_sensitive INTEGER NOT NULL DEFAULT 0,
    use_regex INTEGER NOT NULL DEFAULT 0,
    use_fuzzy INTEGER NOT NULL DEFAULT 0,
    extension TEXT,
    tags TEXT DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_used TEXT,
    use_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_saved_name ON saved_searches(name);
```

## Features

- **Prepared statements** for repeated queries
- **Transaction batching** for bulk inserts
- **Index optimization** for fast lookups
- **WAL mode** for concurrent reads
- **In-memory mode** for testing

## Location

- **Linux**: `~/.local/share/seekr/seekr.db`
- **macOS**: `~/Library/Application Support/com.muhammad-fiaz.seekr/seekr.db`
- **Windows**: `C:\Users\<user>\AppData\Roaming\muhammad-fiaz\seekr\seekr.db`

## In-Memory Mode

For testing, Seekr supports in-memory SQLite databases:

```rust
let db = Database::open_memory()?;
```

## API Examples

### Recording Search History

```rust
db.record_search("*.rs", false, false, false, 42)?;
let history = db.get_history(10)?;
```

### Managing Saved Searches

```rust
let query = SearchQuery {
    pattern: "*.rs".into(),
    ..SearchQuery::default()
};
db.save_search("rust-files", Some("Find Rust files"), &query, &["rust".into()])?;
db.touch_saved_search("rust-files")?;
```
