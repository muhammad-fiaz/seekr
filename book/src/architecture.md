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
| `content_search` | Grep-like content search within files |
| `history` | Search history tracking |
| `saved_searches` | Save and load search queries |
| `plugin` | Plugin system with lifecycle hooks |
| `ranking` | Custom ranking algorithms (TF-IDF, BM25, Combined) |
| `distributed` | Distributed indexing across nodes |
| `network` | Network file search (SMB/CIFS) |
| `collaboration` | Real-time collaboration, shared sessions |
| `analytics` | Advanced search and file type analytics |
| `web` | HTTP API for web UI integration |
| `ml` | Machine learning-based relevance scoring |
| `semantic` | TF-IDF semantic search |
| `i18n` | Multi-language support (10 languages) |
| `mobile` | Mobile companion API with pagination |
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
│   ├── lib.rs              # Library root
│   ├── main.rs             # CLI entry point
│   ├── types.rs            # Domain models
│   ├── error.rs            # Error types
│   ├── database.rs         # SQLite persistence
│   ├── indexer.rs          # Filesystem traversal
│   ├── search.rs           # Query engine
│   ├── content_search.rs   # Grep-like content search
│   ├── history.rs          # Search history tracking
│   ├── saved_searches.rs   # Saved search queries
│   ├── plugin.rs           # Plugin system
│   ├── ranking.rs          # Custom ranking algorithms
│   ├── distributed.rs      # Distributed indexing
│   ├── network.rs          # Network file search
│   ├── collaboration.rs    # Real-time collaboration
│   ├── analytics.rs        # Advanced analytics
│   ├── web.rs              # HTTP API server
│   ├── ml.rs               # ML-based relevance
│   ├── semantic.rs         # Semantic search
│   ├── i18n.rs             # Multi-language support
│   ├── mobile.rs           # Mobile companion API
│   ├── watcher.rs          # Filesystem monitoring
│   ├── platform.rs         # OS abstractions
│   ├── config.rs           # Configuration management
│   ├── cache.rs            # Caching layer
│   ├── core.rs             # Application facade
│   └── cli.rs              # CLI definitions, dispatch & output
├── examples/               # Feature demos
├── benches/                # Benchmarks
├── book/                   # mdBook documentation
└── .github/                # CI/CD workflows
```

## Data Flow

1. **Indexing**: `indexer` walks the filesystem, extracts metadata, stores in `database`
2. **Searching**: `search` queries `database`, applies filters, ranks results
3. **Content Search**: `content_search` reads file contents and matches patterns (grep-like)
4. **ML Scoring**: `ml` applies linear model for relevance scoring
5. **Semantic Search**: `semantic` computes TF-IDF similarity between queries and files
6. **Watching**: `watcher` monitors changes, triggers incremental `indexer` updates
7. **Caching**: `cache` stores hot search results for faster repeated queries
8. **History**: `history` records search queries for review
9. **Analytics**: `analytics` collects search and file type metrics

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
| `chrono` | Date/time handling |
| `thiserror` | Error type derivation |
| `anyhow` | Error context handling |
