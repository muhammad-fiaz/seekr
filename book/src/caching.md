# Caching

## Cache Architecture

Seekr uses a multi-tier caching system built on `moka`:

| Cache | Purpose | Default TTL |
|-------|---------|-------------|
| Search Cache | Search results | 3600s |
| Metadata Cache | File entries | 1800s |
| Index Cache | Statistics | 300s |

## Configuration

```toml
cache_enabled = true
cache_ttl = 3600
```

## Cache Operations

The cache is automatically managed:

- **Hits**: Cached results are returned instantly
- **Misses**: Results are computed and cached
- **Invalidation**: Cache is cleared on index updates

## Disabling Cache

```bash
# Via environment variable
SEEKR_CACHE_ENABLED=false seekr search "query"

# Via configuration
# Set cache_enabled = false in seekr.toml
```

## Clearing Cache

Cache is automatically cleared when:
- The index is rebuilt (`seekr reindex`)
- Stale entries are removed (`seekr doctor`)
- The application calls `clear_caches()`
