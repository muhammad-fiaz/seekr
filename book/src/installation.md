# Installation

## Prerequisites

Before installing Seekr, ensure you have the following tools installed:

- **Rust**: v1.85.0+ (Rust 2024 Edition). Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo**: Comes bundled with Rust. Verify with `cargo --version`.
- **Git**: Required for cloning the repository. Install from [git-scm.com](https://git-scm.com/) or via your package manager:
  ```bash
  # Ubuntu/Debian
  sudo apt install git

  # macOS (with Homebrew)
  brew install git

  # Windows (with Chocolatey)
  choco install git
  ```

### Verify Prerequisites

```bash
rustc --version    # Should show 1.85.0+
cargo --version    # Should show cargo 1.85.0+
git --version      # Any recent version
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
