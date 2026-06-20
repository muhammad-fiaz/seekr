# Performance

## Benchmarks

Performance measured with `cargo bench` on Windows (x86_64):

| Benchmark | Time | Description |
|-----------|------|-------------|
| `full_index_5000_files` | **~75 ms** | Index 5,000 files with parallel traversal |
| `incremental_index` | **~84 ms** | Incremental index update |
| `index_directory` | **~71 ms** | Index a real-world project directory |
| `search_filename` | **~722 ns** | Filename search (< 1 microsecond) |
| `search_regex` | **~19 us** | Regex pattern search |
| `search_fuzzy` | **~3.0 ms** | Fuzzy matching with scoring |
| `tfidf_ranking_1000` | **~326 us** | TF-IDF ranking over 1,000 files |
| `bm25_ranking_1000` | **~317 us** | BM25 ranking over 1,000 files |
| `combined_ranking_1000` | **~445 us** | Combined ranking over 1,000 files |
| `ml_score_100_entries` | **~30 us** | ML relevance scoring for 100 files |
| `semantic_similarity_500` | **~830 us** | Semantic similarity for 500 files |

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
