# Introduction

**Seekr** is a fast, local-first, privacy-focused file search engine and command-line utility written in Rust.

Seekr is designed to be a serious alternative to operating system file search utilities while remaining lightweight, efficient, maintainable, and completely offline.

## Key Principles

- **Library-first architecture** - All functionality is exposed as a reusable Rust library.
- **CLI as interface only** - The CLI is a thin layer over the library.
- **Strong separation of concerns** - Each module has a single, well-defined responsibility.
- **Performance-first** - Parallel indexing, efficient queries, and minimal allocations.
- **Privacy by design** - Zero telemetry, no analytics, no external services.
- **Cross-platform** - Native support for Windows, Linux, and macOS.

## Why Seekr?

Most operating system file search utilities are slow, limited, and often send data to external services. Seekr provides:

- **Speed** - Parallel indexing with Rayon, SQLite-backed persistence, and multi-tier caching.
- **Power** - Regex, fuzzy search, filters, sorting, content search (grep), ML-based relevance, and semantic search.
- **Privacy** - Everything stays on your machine. No cloud, no telemetry.
- **Modularity** - Use the CLI or integrate the library into your own Rust projects.
- **Extensibility** - Plugin system with lifecycle hooks for custom behavior.
- **Collaboration** - Shared search sessions and real-time collaboration features.
- **Analytics** - Advanced search and file type analytics for insights.
- **Internationalization** - Multi-language support (10 languages).
- **Mobile-ready** - Lightweight API optimized for mobile clients.

## Features

| Category | Features |
|----------|----------|
| **Search** | Filename, path, regex, fuzzy, content (grep), ML relevance, semantic |
| **Indexing** | Parallel, incremental, distributed across nodes |
| **Filtering** | Extension, size, date, hidden files, directories |
| **Sorting** | Relevance, name, path, size, modified, extension |
| **Persistence** | SQLite backend with WAL mode |
| **Caching** | Multi-tier caching with TTL |
| **Export** | JSON, CSV, pretty-print formats |
| **Collaboration** | Shared sessions, bookmarks |
| **Analytics** | Search patterns, file type distribution |
| **i18n** | 10 languages (EN, ES, FR, DE, JA, ZH, PT, RU, AR, HI) |
| **Platform** | Windows, Linux, macOS native support |
