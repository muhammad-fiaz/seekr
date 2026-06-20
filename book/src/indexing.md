# Indexing

## How Indexing Works

Seekr uses `walkdir` for filesystem traversal and `rayon` for parallel processing.

### Steps

1. Walk the directory tree
2. Filter ignored directories and patterns
3. Extract metadata (size, dates, extension)
4. Build `FileEntry` objects
5. Batch insert into SQLite

## Configuration

```toml
[indexer]
ignore_dirs = [".git", "node_modules", "target"]
ignore_patterns = ["*.pyc", "*.o"]
follow_links = false
max_depth = 20
max_file_size = 1073741824
```

## Incremental Indexing

Only re-indexes files modified since a given timestamp:

```bash
seekr index . --incremental
```

## Parallel Processing

Indexing uses Rayon's parallel bridge to process entries across multiple threads:

```rust
use rayon::prelude::*;
use walkdir::WalkDir;

let entries: Vec<FileEntry> = WalkDir::new(root)
    .into_iter()
    .par_bridge()
    .filter_map(|e| build_entry(&e.ok()?, &config))
    .collect();
```

## Stale Entry Removal

```bash
seekr doctor
```

This scans the index and removes entries for files that no longer exist.
