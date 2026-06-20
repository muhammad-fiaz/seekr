# Searching

## Search Types

### Filename Search
```bash
seekr search "report.pdf"
```

### Path Search
```bash
seekr search "Documents/report"
```

### Regex Search
```bash
seekr search '\.rs$' --regex
seekr search '^[a-z]+\.(txt|md)$' --regex
```

### Fuzzy Search
```bash
seekr search "cnfg" --fuzzy
seekr search "mn.rs" --fuzzy
```

## Content Search (Grep)

Search within file contents like grep, with parallel file reading:

```bash
# Basic content search
seekr grep "fn main"

# Case-sensitive
seekr grep "TODO" --case-sensitive

# Filter by extension
seekr grep "error" --extension rs

# Regex pattern
seekr grep "fn \w+" --regex

# Limit results
seekr grep "pattern" --limit 10

# Restrict to a subdirectory
seekr grep "error" --root src/

# Max file size filter
seekr grep "pattern" --max-size 1048576
```

### Grep Options

| Option | Description |
|--------|-------------|
| `--root <PATH>` | Restrict search to files under this directory |
| `-S, --case-sensitive` | Case-sensitive matching |
| `-r, --regex` | Use regex matching |
| `-x, --extension <EXT>` | Filter by file extension |
| `--max-size <BYTES>` | Maximum file size to search |
| `-l, --limit <N>` | Maximum number of results |

### How Grep Works

1. Reads all indexed files from the database (up to 100K files)
2. Filters by extension, max size, and root path
3. Reads each file in parallel using Rayon
4. Searches line-by-line for the pattern
5. Reports file path, line number, and matched content

## ML-Based Search

Machine learning-based relevance scoring using a linear model with 8 features:

```bash
# Basic ML search
seekr ml-search "configuration file"

# Limit results
seekr ml-search "error handling" --limit 10
```

### How ML Search Works

The ML model scores files based on:

| Feature | Weight | Description |
|---------|--------|-------------|
| `name_match` | 0.8 | Whether the filename contains the pattern |
| `path_match` | 0.5 | Whether the path contains the pattern |
| `name_length` | 0.3 | Shorter names score higher |
| `is_hidden` | -0.5 | Hidden files penalized |
| `days_since_modified` | 0.15 | Newer files score higher |
| `has_extension` | 0.2 | Files with extensions score higher |
| `size_log` | 0.1 | Log-scaled file size |
| `path_depth` | 0.05 | Shallow paths slightly preferred |

Scores are normalized to 0-100 range.

## Semantic Search

TF-IDF based semantic similarity search:

```bash
# Basic semantic search
seekr semantic "error handling"

# Limit results
seekr semantic "file operations" --limit 20
```

### How Semantic Search Works

1. Builds a TF-IDF vocabulary from all indexed file names
2. Encodes each file name as a TF-IDF vector
3. Computes cosine similarity between query and each file
4. Returns files ranked by semantic similarity

The semantic encoder is built lazily on first query and rebuilt automatically when the index is updated.

## Filters

### Extension Filter
```bash
seekr search "." --extension rs
```

### Size Filter
```bash
seekr search "." --min-size 1024 --max-size 1048576
```

### Hidden Files
```bash
seekr search "." --hidden
```

### Directories
```bash
seekr search "." --dirs
```

## Sorting

```bash
seekr search "." --sort name --order asc
seekr search "." --sort size --order desc
seekr search "." --sort modified
```

Sort fields: `relevance`, `name`, `path`, `size`, `modified`, `extension`

## Output Formats

```bash
seekr search "." --format pretty   # Default colored output
seekr search "." --format json     # JSON output
seekr search "." --format csv      # CSV output
seekr search "." --output results.json  # Export to file
```

All search commands (search, grep, ml-search, semantic) support `--format` and `--output`.

## Pagination

```bash
seekr search "." --limit 10        # First 10 results
```

## Search History

Track and review past searches:

```bash
seekr history list                 # Show recent searches
seekr history list --limit 50      # Show more entries
seekr history clear                # Clear all history
```

## Saved Searches

Save frequently used searches:

```bash
# Save a search
seekr saved save "rust-files" "*.rs" --tags "rust,code"

# List saved searches
seekr saved list

# Load and execute a saved search
seekr saved load "rust-files"

# Delete a saved search
seekr saved delete "rust-files"
```
