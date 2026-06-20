# Performance

## Benchmarks

Performance measured with `cargo bench` on Windows (x86_64):

| Benchmark | Time | Description |
|-----------|------|-------------|
| `full_index_5000_files` | **~62 ms** | Index 5,000 files with parallel traversal |
| `incremental_index` | **~95 ms** | Incremental index update |
| `index_directory` | **~73 ms** | Index a real-world project directory |
| `search_filename` | **~828 ns** | Filename search (< 1 microsecond) |
| `search_regex` | **~21 us** | Regex pattern search |
| `search_fuzzy` | **~2.8 ms** | Fuzzy matching with scoring |

Run benchmarks yourself:

```bash
cargo bench
```

Or use the CLI benchmark command:

```bash
seekr benchmark . --iterations 5
```

## Optimization Strategies

### Parallel Indexing
- Uses `rayon::par_bridge()` for multi-threaded traversal
- Processes entries in parallel across available CPU cores
- Significant speedup on multi-core systems

### Efficient Queries
- Prepared SQL statements for repeated queries
- SQLite indexes on frequently queried columns
- Batch inserts via transactions

### Caching
- Multi-tier caching with Moka
- TTL-based expiration (default: 3600s)
- Automatic invalidation on index updates
- Search result caching for repeated queries

### Minimal Allocations
- String interning for repeated values
- Lazy evaluation where possible
- Zero-copy deserialization for cached data

## Performance Tips

1. **Use incremental indexing** for regular updates (`seekr index . --incremental`)
2. **Enable caching** for repeated searches (enabled by default)
3. **Set max_depth** to limit traversal depth
4. **Configure ignore_dirs** to skip irrelevant directories (`.git`, `node_modules`, etc.)
5. **Use extension filters** to narrow search scope (`--extension rs`)

## Scaling

Seekr is designed to handle:
- Millions of indexed files
- Sub-second search response times
- Fast startup (< 100ms for warm cache)
- Efficient memory usage
