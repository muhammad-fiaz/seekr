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
