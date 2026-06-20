# Architecture

## Library-First Design

Seekr is built as a Rust library with a CLI on top. All functionality is available through the library API.

```
┌─────────────────────────────────────────┐
│                  CLI                     │
│             (cli.rs)                    │
├─────────────────────────────────────────┤
│              Core Facade                │
│             (core::SeekrApp)            │
├──────┬──────┬──────┬──────┬──────┬──────┤
│search│index │watch │cache │config│ plat │
│      │      │      │      │      │ form │
├──────┴──────┴──────┴──────┴──────┴──────┤
│              Database                   │
│           (rusqlite/SQLite)             │
├─────────────────────────────────────────┤
│              Types                      │
│        (shared domain models)           │
└─────────────────────────────────────────┘
```

## Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `types` | Domain models, structures, enums, traits |
| `error` | Unified error types (`thiserror`) |
| `database` | SQLite persistence, migrations, queries |
| `indexer` | Filesystem traversal, metadata extraction |
| `search` | Query engine, filtering, ranking |
| `watcher` | Filesystem monitoring, event processing |
| `platform` | OS-specific abstractions |
| `config` | Configuration loading, validation |
| `cache` | Multi-tier caching with TTL (`moka`) |
| `core` | Application facade, service coordination |
| `cli` | Argument parsing, dispatch, output rendering |

## Project Structure

```
seekr/
├── src/
│   ├── lib.rs          # Library root
│   ├── main.rs         # CLI entry point
│   ├── types.rs        # Domain models
│   ├── error.rs        # Error types
│   ├── database.rs     # SQLite persistence
│   ├── indexer.rs      # Filesystem traversal
│   ├── search.rs       # Query engine
│   ├── watcher.rs      # Filesystem monitoring
│   ├── platform.rs     # OS abstractions
│   ├── config.rs       # Configuration management
│   ├── cache.rs        # Caching layer
│   ├── core.rs         # Application facade
│   └── cli.rs          # CLI definitions, dispatch & output
├── examples/           # Feature demos
├── benches/            # Benchmarks
├── book/               # mdBook documentation
└── .github/            # CI/CD workflows
```

## Data Flow

1. **Indexing**: `indexer` walks the filesystem, extracts metadata, stores in `database`
2. **Searching**: `search` queries `database`, applies filters, ranks results
3. **Watching**: `watcher` monitors changes, triggers incremental `indexer` updates
4. **Caching**: `cache` stores hot search results for faster repeated queries

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `rusqlite` | SQLite database (bundled) |
| `walkdir` | Recursive directory traversal |
| `rayon` | Parallel processing |
| `notify` | Filesystem event monitoring |
| `moka` | Concurrent caching with TTL |
| `clap` | CLI argument parsing |
| `serde` | Serialization/deserialization |
| `regex` | Regular expression support |
| `fuzzy-matcher` | Fuzzy string matching |
| `thiserror` | Error type derivation |
| `anyhow` | Error context handling |
