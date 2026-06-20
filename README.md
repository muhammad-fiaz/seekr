<div align="center">

# Seekr

<a href="https://muhammad-fiaz.github.io/seekr/"><img src="https://img.shields.io/badge/docs-muhammad--fiaz.github.io-blue" alt="Documentation"></a>
<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-2024_Edition-orange.svg" alt="Rust Version"></a>
<a href="https://github.com/muhammad-fiaz/seekr"><img src="https://img.shields.io/github/stars/muhammad-fiaz/seekr" alt="GitHub stars"></a>
<a href="https://github.com/muhammad-fiaz/seekr/issues"><img src="https://img.shields.io/github/issues/muhammad-fiaz/seekr" alt="GitHub issues"></a>
<a href="https://github.com/muhammad-fiaz/seekr/pulls"><img src="https://img.shields.io/github/issues-pr/muhammad-fiaz/seekr" alt="GitHub pull requests"></a>
<a href="https://github.com/muhammad-fiaz/seekr"><img src="https://img.shields.io/github/last-commit/muhammad-fiaz/seekr" alt="GitHub last commit"></a>
<a href="https://github.com/muhammad-fiaz/seekr"><img src="https://img.shields.io/github/license/muhammad-fiaz/seekr" alt="License"></a>
<a href="https://github.com/muhammad-fiaz/seekr/actions/workflows/ci.yml"><img src="https://github.com/muhammad-fiaz/seekr/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
<img src="https://img.shields.io/badge/platforms-linux%20%7C%20windows%20%7C%20macos-blue" alt="Supported Platforms">
<a href="https://github.com/muhammad-fiaz/seekr/releases/latest"><img src="https://img.shields.io/github/v/release/muhammad-fiaz/seekr?label=Latest%20Release&style=flat-square" alt="Latest Release"></a>
<a href="https://pay.muhammadfiaz.com"><img src="https://img.shields.io/badge/Sponsor-pay.muhammadfiaz.com-ff69b4?style=flat&logo=heart" alt="Sponsor"></a>
<a href="https://github.com/sponsors/muhammad-fiaz"><img src="https://img.shields.io/badge/Sponsor-💖-pink?style=social&logo=github" alt="GitHub Sponsors"></a>
<a href="https://hits.sh/muhammad-fiaz/seekr/"><img src="https://hits.sh/muhammad-fiaz/seekr.svg?label=Visitors&extraCount=0&color=green" alt="Repo Visitors"></a>

<p><em>A fast, local-first, privacy-focused file search engine and command-line utility written in Rust.</em></p>

<b><a href="https://muhammad-fiaz.github.io/seekr/">Documentation</a> |
<a href="https://muhammad-fiaz.github.io/seekr/api">API Reference</a> |
<a href="CONTRIBUTING.md">Contributing</a></b>

</div>

A Rust-powered, cross-platform file search engine designed with a clean, modular, library-first architecture. Completely offline with zero telemetry.

**If you love `Seekr`, make sure to give it a star!**

---

<details>
<summary><strong>Table of Contents</strong> (click to expand)</summary>

- [Prerequisites & Supported Platforms](#prerequisites--supported-platforms)
- [Features of Seekr](#features-of-seekr)
- [Installation](#installation)
  - [Install via Cargo](#install-via-cargo-recommended-cli-installation)
  - [Build from Source](#build-from-source)
- [Library Usage](#library-usage)
- [CLI Usage](#cli-usage)
  - [Indexing Files](#indexing-files)
  - [Searching Files](#searching-files)
  - [Watching Directories](#watching-directories)
  - [File Operations](#file-operations)
- [Configuration](#configuration)
- [Benchmarks](#benchmarks)
- [Architecture](#architecture)
- [Examples](#examples)
- [License](#license)

</details>

---

<details>
<summary><strong>Features of Seekr</strong> (click to expand)</summary>

| Feature | Description |
|---------|-------------|
| **Filename Search** | Search files by name with exact, wildcard, or partial matching. |
| **Path Search** | Search within full file paths for precise location-based queries. |
| **Regex Search** | Use regular expressions for advanced pattern matching. |
| **Fuzzy Search** | Find files with fuzzy matching for typo-tolerant queries. |
| **Extension Filter** | Filter results by file extension (e.g., `.rs`, `.txt`). |
| **Size Filters** | Filter files by minimum and maximum size. |
| **Date Filters** | Filter by modification date ranges. |
| **Hidden File Control** | Include or exclude hidden files from results. |
| **Parallel Indexing** | Multi-threaded filesystem traversal with Rayon. |
| **Incremental Indexing** | Only re-index modified files for fast updates. |
| **SQLite Backend** | Persistent, performant index storage with prepared statements. |
| **Filesystem Watching** | Real-time monitoring of directory changes. |
| **Caching Layer** | Multi-tier caching with TTL support via Moka. |
| **JSON/CSV Export** | Export search results in multiple formats. |
| **Cross-Platform** | Native support for Windows, Linux, and macOS. |
| **Library + CLI** | Use as a Rust library or a standalone CLI tool. |
| **Zero Telemetry** | No analytics, tracking, or external services. |

</details>

---

<details>
<summary><strong>Prerequisites & Supported Platforms</strong> (click to expand)</summary>

## Prerequisites

Before installing Seekr, ensure you have:
- **Rust Toolchain**: v1.85.0+ (supports Rust 2024 Edition)

## Supported Platforms

Seekr supports a wide range of platforms and architectures:
- **Windows 10+ / 11+** (Uses Win32 API for file operations)
- **Linux** (POSIX systems)
- **macOS** (UNIX systems, Finder integration)

</details>

---

## Installation

### Install via Cargo (Recommended CLI Installation)
Ensure you have the Rust toolchain installed. You can install the `seekr` executable directly from the GitHub repository using:

```bash
cargo install --git https://github.com/muhammad-fiaz/seekr
```

### Build from Source
```bash
git clone https://github.com/muhammad-fiaz/seekr.git
cd seekr
cargo build --release
```
The compiled binary will be located at `target/release/seekr`.

---

## Library Usage

To use `seekr` as a modular library in your Rust project, reference it in your dependencies:

```rust
use seekr::core::SeekrApp;
use seekr::types::SearchQuery;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the application
    let app = SeekrApp::default_config()?;

    // 2. Index the current directory
    let stats = app.index(Path::new("."))?;
    println!("Indexed {} files, {} directories", stats.total_files, stats.total_dirs);

    // 3. Search for files
    let query = SearchQuery {
        pattern: "main.rs".into(),
        ..SearchQuery::default()
    };
    let results = app.search(&query)?;

    for result in &results {
        println!("{} (score: {:.0})", result.entry.path.display(), result.score);
    }

    Ok(())
}
```

---

## CLI Usage

### Indexing Files
Index the current directory for fast searching:
```bash
seekr index .
```

Index a specific directory with options:
```bash
seekr index /path/to/project --follow-links --max-depth 5
```

Incremental indexing (only modified files):
```bash
seekr index . --incremental
```

### Searching Files
Search for files by name:
```bash
seekr search "main.rs"
```

Case-sensitive search:
```bash
seekr search "README" --case-sensitive
```

Regex search:
```bash
seekr search '\.rs$' --regex
```

Fuzzy search:
```bash
seekr search "mn.rs" --fuzzy
```

Filter by extension:
```bash
seekr search "." --extension rs
```

Filter by size:
```bash
seekr search "." --min-size 1024 --max-size 1048576
```

JSON output:
```bash
seekr search "config" --format json --output results.json
```

### Watching Directories
Watch a directory for changes:
```bash
seekr watch .
```

Watch without recursion:
```bash
seekr watch /path/to/dir --no-recursive
```

### File Operations
Open a file with the default application:
```bash
seekr open /path/to/file.txt
```

Open the containing directory:
```bash
seekr open-dir /path/to/file.txt
```

Reveal a file in the file manager:
```bash
seekr reveal /path/to/file.txt
```

### Other Commands
Show index statistics:
```bash
seekr stats .
```

Run diagnostics:
```bash
seekr doctor
```

Rebuild the index:
```bash
seekr reindex .
```

Run benchmarks:
```bash
seekr benchmark . --iterations 5
```

Show version:
```bash
seekr version
```

Manage configuration:
```bash
seekr config show
seekr config init
```

---

## Configuration

You can customize defaults by placing a `seekr.toml` file in the working directory:

```toml
search_root = "/home/user/projects"
cache_enabled = true
cache_ttl = 3600
default_limit = 50
color = true

[indexer]
follow_links = false
max_depth = 20
max_file_size = 1073741824

[indexer]
ignore_dirs = [".git", "node_modules", "target", "__pycache__"]
ignore_patterns = ["*.pyc", "*.o", "*.so"]
```

Environment variables:
- `SEEKR_CONFIG` - Path to configuration file
- `SEEKR_SEARCH_ROOT` - Default search root
- `SEEKR_CACHE_TTL` - Cache time-to-live in seconds
- `SEEKR_DEFAULT_LIMIT` - Default result limit
- `SEEKR_COLOR` - Enable/disable color output
- `SEEKR_CACHE_ENABLED` - Enable/disable caching

---

## Benchmarks

Performance measured with `cargo bench` on Windows (x86_64, AMD Ryzen):

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

---

## Architecture

Seekr uses a library-first architecture with strict separation of concerns:

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

---

## Examples

Run any example to see Seekr in action:

```bash
cargo run --example quick_start      # Basic indexing and search
cargo run --example search_modes     # All search modes (regex, fuzzy, filters)
cargo run --example indexing          # Full, incremental, and custom indexing
cargo run --example caching          # Cache layer usage
cargo run --example configuration    # Config loading and validation
cargo run --example export_formats   # JSON, CSV, and pretty export
cargo run --example platform_ops     # Platform directories and file ops
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
