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
- **Power** - Regex, fuzzy search, filters, and sorting.
- **Privacy** - Everything stays on your machine. No cloud, no telemetry.
- **Modularity** - Use the CLI or integrate the library into your own Rust projects.
