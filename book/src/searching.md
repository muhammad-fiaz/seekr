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

Search within file contents like grep:

```bash
seekr grep "fn main"
seekr grep "TODO" --case-sensitive
seekr grep "error" --extension rs
seekr grep "fn \w+" --regex
seekr grep "pattern" --max-size 1048576
```

## ML-Based Search

Machine learning-based relevance scoring:

```bash
seekr ml-search "configuration file"
seekr ml-search "error handling" --limit 10
```

## Semantic Search

TF-IDF based semantic similarity search:

```bash
seekr semantic "error handling"
seekr semantic "file operations" --limit 20
```

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
