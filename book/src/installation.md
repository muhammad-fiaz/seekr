# Installation

## Prerequisites

- **Rust Toolchain**: v1.85.0+ (Rust 2024 Edition)

Install Rust from [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install via Cargo

```bash
cargo install --git https://github.com/muhammad-fiaz/seekr
```

## Build from Source

```bash
git clone https://github.com/muhammad-fiaz/seekr.git
cd seekr
cargo build --release
```

The compiled binary will be at `target/release/seekr`.

## Verify Installation

```bash
seekr version
```

## Platform-Specific Notes

### Windows
- Uses Win32 API for file reveal operations.
- SQLite is bundled via the `bundled` feature.

### Linux
- Uses POSIX APIs for file operations.
- `xdg-open` is used for opening files.

### macOS
- Uses Finder integration for file reveal (`open -R`).
- Native `open` command for opening files.
