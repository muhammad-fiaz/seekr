# Platform Support

## Supported Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| Windows 10/11 | Supported | Win32 API for file operations |
| Linux | Supported | POSIX APIs, xdg-open |
| macOS | Supported | Finder integration, open command |

## Platform-Specific Features

### File Reveal

- **Windows**: `explorer /select,<path>`
- **macOS**: `open -R <path>`
- **Linux**: Opens parent directory

### File Open

All platforms use the `open` crate for cross-platform file opening.

### Configuration Directories

| Platform | Config Path |
|----------|-------------|
| Windows | `C:\Users\<user>\AppData\Roaming\muhammad-fiaz\seekr\` |
| Linux | `~/.config/seekr/` |
| macOS | `~/Library/Application Support/com.muhammad-fiaz.seekr/` |

## Building for Specific Platforms

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target aarch64-apple-darwin
```
