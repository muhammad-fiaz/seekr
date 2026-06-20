# Commands

## Global Flags

These flags apply to all commands:

- `-v, --verbose` - Enable verbose output
- `--no-color` - Suppress color output
- `-f, --format <FORMAT>` - Output format: pretty, json, csv (default: pretty)

## seekr index

Index a directory for fast searching.

```bash
seekr index <PATH> [OPTIONS]
```

**Options:**
- `--incremental` - Only index modified files (last 24 hours)
- `--follow-links` - Follow symbolic links
- `--max-depth <N>` - Maximum directory depth

## seekr search

Search indexed files.

```bash
seekr search <PATTERN> [OPTIONS]
```

**Options:**
- `--root <PATH>` - Search in a specific root directory
- `-S, --case-sensitive` - Case-sensitive matching
- `-r, --regex` - Use regex matching
- `-z, --fuzzy` - Use fuzzy matching
- `-x, --extension <EXT>` - Filter by extension
- `--min-size <BYTES>` - Minimum file size
- `--max-size <BYTES>` - Maximum file size
- `--hidden` - Include hidden files
- `--dirs` - Include directories in results
- `-l, --limit <N>` - Maximum number of results
- `-s, --sort <FIELD>` - Sort by: relevance, name, path, size, modified, extension
- `--order <ORDER>` - Sort order: asc, desc
- `--output <FILE>` - Export results to file (JSON/CSV)

## seekr grep

Search file contents (grep-like). Reads files in parallel for speed.

```bash
seekr grep <PATTERN> [OPTIONS]
```

**Options:**
- `--root <PATH>` - Restrict search to files under this directory
- `-S, --case-sensitive` - Case-sensitive matching
- `-r, --regex` - Use regex matching
- `-x, --extension <EXT>` - Filter by extension
- `--max-size <BYTES>` - Maximum file size to search
- `-l, --limit <N>` - Maximum number of results

## seekr ml-search

Search with ML-based relevance scoring.

```bash
seekr ml-search <PATTERN> [OPTIONS]
```

**Options:**
- `-l, --limit <N>` - Maximum number of results

## seekr semantic

Perform semantic search using TF-IDF similarity.

```bash
seekr semantic <QUERY> [OPTIONS]
```

**Options:**
- `-l, --limit <N>` - Maximum number of results

## seekr history

Manage search history.

```bash
seekr history <ACTION>
```

**Actions:**
- `list` - Show recent search history
- `clear` - Clear all search history

**Options for list:**
- `-l, --limit <N>` - Number of entries to show (default: 20)

## seekr saved

Manage saved searches.

```bash
seekr saved <ACTION>
```

**Actions:**
- `list` - List all saved searches
- `save <NAME> <PATTERN>` - Save a search
- `load <NAME>` - Load and execute a saved search
- `delete <NAME>` - Delete a saved search

**Options for save:**
- `-t, --tags <TAGS>` - Comma-separated tags

## seekr analytics

View analytics and file type distribution.

```bash
seekr analytics [PATH]
```

## seekr watch

Watch a directory for changes and update the index.

```bash
seekr watch <PATH> [OPTIONS]
```

**Options:**
- `--no-recursive` - Do not watch recursively
- `--debounce <MS>` - Debounce interval in milliseconds (default: 500)

## seekr stats

Show index statistics.

```bash
seekr stats [PATH]
```

Displays total files, directories, size, hidden files, unique extensions, and last indexed time.

## seekr doctor

Check the health of the index and remove stale entries.

```bash
seekr doctor
```

## seekr config

Manage configuration.

```bash
seekr config show     # Show current configuration
seekr config init     # Generate default config file (seekr.toml)
seekr config set <KEY> <VALUE>
```

## seekr reindex

Rebuild the entire index from scratch.

```bash
seekr reindex [PATH]
```

Clears the existing index and re-indexes the specified directory.

## seekr benchmark

Run performance benchmarks.

```bash
seekr benchmark [PATH] [OPTIONS]
```

**Options:**
- `-i, --iterations <N>` - Number of iterations (default: 3)

## seekr open

Open a file with the default application.

```bash
seekr open <PATH>
```

## seekr open-dir

Open the containing directory of a file.

```bash
seekr open-dir <PATH>
```

## seekr reveal

Reveal a file in the file manager (Finder on macOS, Explorer on Windows).

```bash
seekr reveal <PATH>
```

## seekr version

Show version information.

```bash
seekr version
```
