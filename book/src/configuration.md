# Configuration

## Configuration File

Seekr uses a TOML configuration file. Place `seekr.toml` in your working directory:

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
ignore_dirs = [".git", "node_modules", "target", "__pycache__"]
ignore_patterns = ["*.pyc", "*.o", "*.so"]
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SEEKR_CONFIG` | Path to config file | Platform default |
| `SEEKR_SEARCH_ROOT` | Default search root | Current directory |
| `SEEKR_CACHE_TTL` | Cache TTL in seconds | 3600 |
| `SEEKR_DEFAULT_LIMIT` | Default result limit | 50 |
| `SEEKR_COLOR` | Enable color output | true |
| `SEEKR_CACHE_ENABLED` | Enable caching | true |

## Priority Order

1. Explicit path argument
2. `SEEKR_CONFIG` environment variable
3. Platform config directory (`~/.config/seekr/seekr.toml`)
4. Current directory (`./seekr.toml`)
5. Built-in defaults
