use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chrono::Utc;
use clap::{Parser, Subcommand};
use colored::*;

use crate::config;
use crate::core::SeekrApp;
use crate::error::SeekrResult;
use crate::platform;
use crate::types::*;

// ---------------------------------------------------------------------------
// CLI Definitions
// ---------------------------------------------------------------------------

/// Seekr - A fast, local-first, privacy-focused file search engine.
#[derive(Parser, Debug)]
#[command(
    name = "seekr",
    about = "A fast, local-first, privacy-focused file search engine and CLI utility.",
    version,
    author = "Muhammad Fiaz",
    long_about = "Seekr is a fast, local-first, privacy-focused file search engine written in Rust.\n\nIt provides efficient file indexing, powerful search capabilities, and real-time\nfilesystem monitoring — all completely offline with zero telemetry."
)]
pub struct Cli {
    /// Enable verbose output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress color output.
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output format.
    #[arg(
        short,
        long,
        global = true,
        value_enum,
        default_value_t = OutputFormatArg::Pretty
    )]
    pub format: OutputFormatArg,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Index a directory for fast searching.
    Index {
        /// The directory to index.
        #[arg(default_value = ".")]
        path: String,

        /// Perform incremental indexing.
        #[arg(short, long)]
        incremental: bool,

        /// Follow symbolic links.
        #[arg(long)]
        follow_links: bool,

        /// Maximum directory depth.
        #[arg(short, long)]
        max_depth: Option<usize>,
    },

    /// Search indexed files.
    Search {
        /// The search pattern.
        pattern: String,

        /// Search in a specific root directory.
        #[arg(long)]
        root: Option<String>,

        /// Case-sensitive search.
        #[arg(short = 'S', long)]
        case_sensitive: bool,

        /// Use regex matching.
        #[arg(short, long)]
        regex: bool,

        /// Use fuzzy matching.
        #[arg(short = 'z', long)]
        fuzzy: bool,

        /// Filter by file extension.
        #[arg(short = 'x', long)]
        extension: Option<String>,

        /// Minimum file size in bytes.
        #[arg(long)]
        min_size: Option<u64>,

        /// Maximum file size in bytes.
        #[arg(long)]
        max_size: Option<u64>,

        /// Include hidden files.
        #[arg(long)]
        hidden: bool,

        /// Include directories in results.
        #[arg(long)]
        dirs: bool,

        /// Maximum number of results.
        #[arg(short, long)]
        limit: Option<usize>,

        /// Sort field.
        #[arg(
            short,
            long,
            value_enum,
            default_value_t = SortArg::Relevance
        )]
        sort: SortArg,

        /// Sort direction.
        #[arg(long, value_enum, default_value_t = OrderArg::Desc)]
        order: OrderArg,

        /// Output file path (JSON).
        #[arg(long)]
        output: Option<String>,
    },

    /// Search file contents (grep-like).
    Grep {
        /// The pattern to search for in file contents.
        pattern: String,

        /// Search in a specific root directory.
        #[arg(long)]
        root: Option<String>,

        /// Case-sensitive search.
        #[arg(short = 'S', long)]
        case_sensitive: bool,

        /// Use regex matching.
        #[arg(short, long)]
        regex: bool,

        /// Filter by file extension.
        #[arg(short = 'x', long)]
        extension: Option<String>,

        /// Maximum file size in bytes.
        #[arg(long)]
        max_size: Option<u64>,

        /// Maximum number of results.
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Search with ML-based relevance scoring.
    MlSearch {
        /// The search pattern.
        pattern: String,

        /// Maximum number of results.
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Perform semantic search using TF-IDF similarity.
    Semantic {
        /// The search query.
        query: String,

        /// Maximum number of results.
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Manage search history.
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },

    /// Manage saved searches.
    Saved {
        #[command(subcommand)]
        action: SavedAction,
    },

    /// View analytics and statistics.
    Analytics {
        /// The root path to analyze.
        #[arg(default_value = ".")]
        path: String,
    },

    /// Watch a directory for changes and update the index.
    Watch {
        /// The directory to watch.
        #[arg(default_value = ".")]
        path: String,

        /// Do not watch recursively.
        #[arg(long)]
        no_recursive: bool,

        /// Debounce interval in milliseconds.
        #[arg(long, default_value = "500")]
        debounce: u64,
    },

    /// Show index statistics.
    Stats {
        /// The root path to show stats for.
        #[arg(default_value = ".")]
        path: String,
    },

    /// Check the health of the index.
    Doctor,

    /// Manage configuration.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Rebuild the entire index.
    Reindex {
        /// The directory to reindex.
        #[arg(default_value = ".")]
        path: String,
    },

    /// Run performance benchmarks.
    Benchmark {
        /// The directory to benchmark against.
        #[arg(default_value = ".")]
        path: String,

        /// Number of iterations.
        #[arg(short, long, default_value = "3")]
        iterations: u32,
    },

    /// Show version information.
    Version,

    /// Open a file with the default application.
    Open {
        /// The file path to open.
        path: String,
    },

    /// Open the containing directory.
    OpenDir {
        /// The file path whose directory to open.
        path: String,
    },

    /// Reveal a file in the file manager.
    Reveal {
        /// The file path to reveal.
        path: String,
    },
}

/// History subcommands.
#[derive(Subcommand, Debug)]
pub enum HistoryAction {
    /// Show recent search history.
    List {
        /// Number of entries to show.
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Clear all search history.
    Clear,
}

/// Saved searches subcommands.
#[derive(Subcommand, Debug)]
pub enum SavedAction {
    /// List all saved searches.
    List,
    /// Save the current search.
    Save {
        /// Name for the saved search.
        name: String,

        /// Search pattern.
        pattern: String,

        /// Optional tags (comma-separated).
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// Load and execute a saved search.
    Load {
        /// Name of the saved search.
        name: String,
    },
    /// Delete a saved search.
    Delete {
        /// Name of the saved search.
        name: String,
    },
}

/// Configuration subcommands.
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show the current configuration.
    Show,

    /// Generate a default configuration file.
    Init,

    /// Set a configuration value.
    Set {
        /// The configuration key.
        key: String,

        /// The configuration value.
        value: String,
    },
}

/// Output format argument.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum OutputFormatArg {
    Pretty,
    Json,
    Csv,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Pretty => OutputFormat::Pretty,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Csv => OutputFormat::Csv,
        }
    }
}

/// Sort field argument.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum SortArg {
    Relevance,
    Name,
    Path,
    Size,
    Modified,
    Extension,
}

/// Sort order argument.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum OrderArg {
    Asc,
    Desc,
}

// ---------------------------------------------------------------------------
// Command Dispatch
// ---------------------------------------------------------------------------

/// Dispatches a CLI command to the appropriate handler.
pub fn dispatch(cli: &Cli) -> SeekrResult<()> {
    let app = SeekrApp::default_config()?;

    match &cli.command {
        Commands::Index {
            path,
            incremental,
            follow_links,
            max_depth,
        } => cmd_index(&app, path, *incremental, *follow_links, *max_depth),

        Commands::Search {
            pattern,
            root,
            case_sensitive,
            regex,
            fuzzy,
            extension,
            min_size,
            max_size,
            hidden,
            dirs,
            limit,
            sort,
            order,
            output,
        } => cmd_search(
            &app,
            pattern,
            root.as_deref(),
            *case_sensitive,
            *regex,
            *fuzzy,
            extension.as_deref(),
            *min_size,
            *max_size,
            *hidden,
            *dirs,
            *limit,
            *sort,
            *order,
            output.as_deref(),
            cli.format,
        ),

        Commands::Grep {
            pattern,
            root,
            case_sensitive,
            regex,
            extension,
            max_size,
            limit,
        } => cmd_grep(
            &app,
            pattern,
            root.as_deref(),
            *case_sensitive,
            *regex,
            extension.as_deref(),
            *max_size,
            *limit,
            cli.format,
        ),

        Commands::MlSearch { pattern, limit } => cmd_ml_search(&app, pattern, *limit, cli.format),

        Commands::Semantic { query, limit } => cmd_semantic(&app, query, *limit, cli.format),

        Commands::History { action } => cmd_history(&app, action),

        Commands::Saved { action } => cmd_saved(&app, action),

        Commands::Analytics { path } => cmd_analytics(&app, path),

        Commands::Watch {
            path,
            no_recursive,
            debounce,
        } => cmd_watch(path, *no_recursive, *debounce),

        Commands::Stats { path } => cmd_stats(&app, path),

        Commands::Doctor => cmd_doctor(&app),

        Commands::Config { action } => cmd_config(action),

        Commands::Reindex { path } => cmd_reindex(&app, path),

        Commands::Benchmark { path, iterations } => cmd_benchmark(&app, path, *iterations),

        Commands::Version => cmd_version(),

        Commands::Open { path } => cmd_open(path),

        Commands::OpenDir { path } => cmd_open_dir(path),

        Commands::Reveal { path } => cmd_reveal(path),
    }
}

// ---------------------------------------------------------------------------
// Command Handlers
// ---------------------------------------------------------------------------

fn cmd_index(
    app: &SeekrApp,
    path_str: &str,
    incremental: bool,
    follow_links: bool,
    max_depth: Option<usize>,
) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        print_error(&format!("path does not exist: {}", path.display()));
        return Ok(());
    }

    let mut indexer_config = app.config().indexer.clone();
    indexer_config.follow_links = follow_links;
    indexer_config.max_depth = max_depth;

    print_info(&format!("Indexing: {}", path.display()));
    let start = Instant::now();

    let stats = if incremental {
        let since = Utc::now() - chrono::Duration::hours(24);
        app.index_incremental(&path, since)?
    } else {
        app.index_full(&path, &indexer_config)?
    };

    let elapsed = start.elapsed();
    print_success(&format!(
        "Indexed {} files, {} directories ({:.2?})",
        stats.total_files, stats.total_dirs, elapsed
    ));
    print_stats(&stats);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_search(
    app: &SeekrApp,
    pattern: &str,
    root: Option<&str>,
    case_sensitive: bool,
    use_regex: bool,
    use_fuzzy: bool,
    extension: Option<&str>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    include_hidden: bool,
    include_dirs: bool,
    limit: Option<usize>,
    sort: SortArg,
    order: OrderArg,
    output_path: Option<&str>,
    format: OutputFormatArg,
) -> SeekrResult<()> {
    let query = SearchQuery {
        pattern: pattern.to_string(),
        root: root.map(PathBuf::from),
        case_sensitive,
        use_regex,
        use_fuzzy,
        extension: extension.map(String::from),
        min_size,
        max_size,
        include_hidden,
        include_dirs,
        sort_by: match sort {
            SortArg::Relevance => SortField::Relevance,
            SortArg::Name => SortField::Name,
            SortArg::Path => SortField::Path,
            SortArg::Size => SortField::Size,
            SortArg::Modified => SortField::Modified,
            SortArg::Extension => SortField::Extension,
        },
        sort_order: match order {
            OrderArg::Asc => SortOrder::Ascending,
            OrderArg::Desc => SortOrder::Descending,
        },
        limit: Some(limit.unwrap_or(app.config().default_limit)),
        offset: 0,
        ..SearchQuery::default()
    };

    let start = Instant::now();
    let results = app.search(&query)?;
    let elapsed = start.elapsed();

    let out_format = OutputFormat::from(format);

    if let Some(path) = output_path {
        export_results(&results, path, &out_format)?;
        print_success(&format!(
            "Exported {} results to {} ({:.2?})",
            results.len(),
            path,
            elapsed
        ));
    } else {
        match out_format {
            OutputFormat::Json => print_json(&results)?,
            OutputFormat::Csv => print_csv(&results),
            OutputFormat::Pretty => print_results(&results, &elapsed),
        }
    }

    Ok(())
}

fn cmd_watch(path_str: &str, no_recursive: bool, debounce_ms: u64) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        print_error(&format!("path does not exist: {}", path.display()));
        return Ok(());
    }

    let config = WatchConfig {
        path: path.clone(),
        recursive: !no_recursive,
        debounce_ms,
    };

    print_info(&format!("Watching: {}", path.display()));
    print_info("Press Ctrl+C to stop.");

    crate::watcher::watch_and_process(config, |event| match &event {
        FileEvent::Created(p) => print_event("CREATED", &p.to_string_lossy()),
        FileEvent::Modified(p) => print_event("MODIFIED", &p.to_string_lossy()),
        FileEvent::Deleted(p) => print_event("DELETED", &p.to_string_lossy()),
        FileEvent::Renamed { from, to } => {
            print_event(
                "RENAMED",
                &format!("{} -> {}", from.display(), to.display()),
            );
        }
    })?;

    Ok(())
}

fn cmd_stats(app: &SeekrApp, path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);
    let stats = app.stats(&path)?;
    print_stats(&stats);
    Ok(())
}

fn cmd_doctor(app: &SeekrApp) -> SeekrResult<()> {
    print_info("Running diagnostics...");

    let count = app.database().count()?;
    print_success(&format!("Index contains {} entries", count));

    let removed = app.remove_stale()?;
    if removed > 0 {
        print_warning(&format!("Removed {} stale entries", removed));
    } else {
        print_success("No stale entries found");
    }

    print_success("All checks passed");
    Ok(())
}

fn cmd_config(action: &ConfigAction) -> SeekrResult<()> {
    match action {
        ConfigAction::Show => {
            let config = config::load_config(None)?;
            print_config(&config);
        }
        ConfigAction::Init => {
            let content = config::default_config_content();
            let path = PathBuf::from("seekr.toml");
            std::fs::write(&path, &content)?;
            print_success(&format!("Configuration file created: {}", path.display()));
        }
        ConfigAction::Set { key, value } => {
            print_info(&format!("Setting {} = {}", key, value));
            print_warning("Manual configuration editing is currently supported via seekr.toml");
        }
    }
    Ok(())
}

fn cmd_reindex(app: &SeekrApp, path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        print_error(&format!("path does not exist: {}", path.display()));
        return Ok(());
    }

    print_info(&format!("Clearing index for: {}", path.display()));
    app.clear_index()?;

    print_info(&format!("Reindexing: {}", path.display()));
    let start = Instant::now();
    let stats = app.index(&path)?;
    let elapsed = start.elapsed();

    print_success(&format!(
        "Reindexed {} files, {} directories ({:.2?})",
        stats.total_files, stats.total_dirs, elapsed
    ));
    print_stats(&stats);

    Ok(())
}

fn cmd_benchmark(app: &SeekrApp, path_str: &str, iterations: u32) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        print_error(&format!("path does not exist: {}", path.display()));
        return Ok(());
    }

    print_info(&format!("Benchmarking with {} iterations...", iterations));

    let mut index_times = Vec::new();
    let mut search_times = Vec::new();

    for i in 0..iterations {
        app.clear_index()?;

        let start = Instant::now();
        app.index(&path)?;
        let index_time = start.elapsed();
        index_times.push(index_time);

        let query = SearchQuery {
            pattern: "*".into(),
            use_regex: true,
            limit: Some(100),
            ..SearchQuery::default()
        };

        let start = Instant::now();
        let _results = app.search(&query)?;
        let search_time = start.elapsed();
        search_times.push(search_time);

        print_info(&format!(
            "  Iteration {}/{}: index={:.2?}, search={:.2?}",
            i + 1,
            iterations,
            index_time,
            search_time
        ));
    }

    let avg_index: Duration = index_times.iter().sum::<Duration>() / iterations;
    let avg_search: Duration = search_times.iter().sum::<Duration>() / iterations;

    print_success(&format!(
        "Average index time: {:.2?}, Average search time: {:.2?}",
        avg_index, avg_search
    ));

    Ok(())
}

fn cmd_version() -> SeekrResult<()> {
    println!("seekr {}", env!("CARGO_PKG_VERSION"));
    println!("Rust Edition: 2024");
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
    Ok(())
}

fn cmd_open(path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);
    platform::open_file(&path)?;
    print_success(&format!("Opened: {}", path.display()));
    Ok(())
}

fn cmd_open_dir(path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);
    platform::open_containing_directory(&path)?;
    print_success(&format!("Opened directory for: {}", path.display()));
    Ok(())
}

fn cmd_reveal(path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);
    platform::reveal_file(&path)?;
    print_success(&format!("Revealed: {}", path.display()));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_grep(
    app: &SeekrApp,
    pattern: &str,
    _root: Option<&str>,
    case_sensitive: bool,
    use_regex: bool,
    extension: Option<&str>,
    max_size: Option<u64>,
    limit: Option<usize>,
    _format: OutputFormatArg,
) -> SeekrResult<()> {
    let config = crate::content_search::ContentSearchConfig {
        case_sensitive,
        use_regex,
        max_file_size: max_size.or(Some(10 * 1024 * 1024)),
        extension: extension.map(String::from),
        context_before: 0,
        context_after: 0,
        limit,
    };

    let start = Instant::now();
    let results = app.content_search(pattern, &config)?;
    let elapsed = start.elapsed();

    if results.is_empty() {
        print_warning("No content matches found.");
        return Ok(());
    }

    println!(
        "\n{} Found {} matches in {} files ({:.2?})\n",
        "Results:".cyan().bold(),
        results.iter().map(|r| r.total_matches).sum::<usize>(),
        results.len(),
        elapsed
    );

    for result in &results {
        println!(
            "{} {}",
            result.entry.path.display().to_string().white(),
            format!("({} matches)", result.total_matches).dimmed(),
        );
        for m in &result.matches {
            println!(
                "  {}:{} {}",
                m.line_number.to_string().green(),
                m.start_offset.to_string().dimmed(),
                m.line_content,
            );
        }
        println!();
    }

    Ok(())
}

fn cmd_ml_search(
    app: &SeekrApp,
    pattern: &str,
    limit: Option<usize>,
    format: OutputFormatArg,
) -> SeekrResult<()> {
    let query = SearchQuery {
        pattern: pattern.to_string(),
        limit: Some(limit.unwrap_or(50)),
        ..SearchQuery::default()
    };

    let start = Instant::now();
    let results = app.ml_search(&query)?;
    let elapsed = start.elapsed();

    match format {
        OutputFormatArg::Json => print_json(&results)?,
        OutputFormatArg::Csv => print_csv(&results),
        OutputFormatArg::Pretty => print_results(&results, &elapsed),
    }

    Ok(())
}

fn cmd_semantic(
    app: &SeekrApp,
    query_str: &str,
    limit: Option<usize>,
    format: OutputFormatArg,
) -> SeekrResult<()> {
    let query = SearchQuery {
        pattern: query_str.to_string(),
        limit: Some(limit.unwrap_or(50)),
        ..SearchQuery::default()
    };

    print_info("Building semantic index (first run may take a moment)...");
    let start = Instant::now();
    let results = app.semantic_search(&query)?;
    let elapsed = start.elapsed();

    match format {
        OutputFormatArg::Json => print_json(&results)?,
        OutputFormatArg::Csv => print_csv(&results),
        OutputFormatArg::Pretty => print_results(&results, &elapsed),
    }

    Ok(())
}

fn cmd_history(app: &SeekrApp, action: &HistoryAction) -> SeekrResult<()> {
    match action {
        HistoryAction::List { limit } => {
            let entries = app.get_history(*limit)?;
            if entries.is_empty() {
                print_warning("No search history found.");
                return Ok(());
            }
            println!("\n{}", "Search History:".cyan().bold());
            for entry in &entries {
                let flags = [
                    if entry.case_sensitive { "CS" } else { "" },
                    if entry.use_regex { "RX" } else { "" },
                    if entry.use_fuzzy { "FZ" } else { "" },
                ];
                let flags: Vec<&str> = flags.iter().copied().filter(|s| !s.is_empty()).collect();
                let flags_str = if flags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", flags.join(","))
                };

                println!(
                    "  {} {} ({} results){}",
                    entry.timestamp.format("%Y-%m-%d %H:%M"),
                    entry.pattern.green(),
                    entry.result_count,
                    flags_str.dimmed(),
                );
            }
            println!();
        }
        HistoryAction::Clear => {
            app.clear_history()?;
            print_success("Search history cleared.");
        }
    }
    Ok(())
}

fn cmd_saved(app: &SeekrApp, action: &SavedAction) -> SeekrResult<()> {
    match action {
        SavedAction::List => {
            let searches = app.list_saved_searches()?;
            if searches.is_empty() {
                print_warning("No saved searches found.");
                return Ok(());
            }
            println!("\n{}", "Saved Searches:".cyan().bold());
            for s in &searches {
                let tags_str = if s.tags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", s.tags.join(", "))
                };
                println!(
                    "  {} {} (used {} times){}",
                    s.name.green(),
                    format!("'{}'", s.pattern).dimmed(),
                    s.use_count,
                    tags_str.dimmed(),
                );
            }
            println!();
        }
        SavedAction::Save {
            name,
            pattern,
            tags,
        } => {
            let query = SearchQuery {
                pattern: pattern.to_string(),
                ..SearchQuery::default()
            };
            let tag_list: Vec<String> = tags
                .as_ref()
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();
            app.save_search(name, None, &query, &tag_list)?;
            print_success(&format!("Search '{}' saved.", name));
        }
        SavedAction::Load { name } => {
            if let Some(saved) = app.load_saved_search(name)? {
                app.database().touch_saved_search(name)?;
                let query = SearchQuery {
                    pattern: saved.pattern,
                    case_sensitive: saved.case_sensitive,
                    use_regex: saved.use_regex,
                    use_fuzzy: saved.use_fuzzy,
                    extension: saved.extension,
                    ..SearchQuery::default()
                };
                let start = Instant::now();
                let results = app.search(&query)?;
                let elapsed = start.elapsed();
                print_results(&results, &elapsed);
            } else {
                print_error(&format!("Saved search '{}' not found.", name));
            }
        }
        SavedAction::Delete { name } => {
            if app.delete_saved_search(name)? {
                print_success(&format!("Saved search '{}' deleted.", name));
            } else {
                print_error(&format!("Saved search '{}' not found.", name));
            }
        }
    }
    Ok(())
}

fn cmd_analytics(app: &SeekrApp, path_str: &str) -> SeekrResult<()> {
    let path = PathBuf::from(path_str);
    let report = app.analytics_report(&path)?;

    println!("\n{}", "Analytics Report:".cyan().bold());
    println!(
        "  {} {}",
        "Generated:".white(),
        report.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!();

    println!("{}", "Index Statistics:".cyan().bold());
    println!(
        "  {} {}",
        "Files:".white(),
        report.index_stats.total_files.to_string().green()
    );
    println!(
        "  {} {}",
        "Directories:".white(),
        report.index_stats.total_dirs.to_string().green()
    );
    println!(
        "  {} {}",
        "Total Size:".white(),
        format_size(report.index_stats.total_size).yellow()
    );
    println!();

    println!("{}", "File Type Distribution:".cyan().bold());
    for (ext, count) in &report.file_types.top_extensions {
        let size = report
            .file_types
            .extension_sizes
            .get(ext)
            .copied()
            .unwrap_or(0);
        println!(
            "  {} {} files ({})",
            format!(".{}", ext).green(),
            count,
            format_size(size).yellow()
        );
    }
    println!();

    println!("{}", "Search Analytics:".cyan().bold());
    println!(
        "  {} {}",
        "Total Searches:".white(),
        report.search.total_searches
    );
    println!(
        "  {} {:.2}ms",
        "Avg Duration:".white(),
        report.search.avg_search_duration_ms
    );
    println!(
        "  {} {}",
        "No Results:".white(),
        report.search.results_distribution.no_results
    );
    println!(
        "  {} {}",
        "Few Results:".white(),
        report.search.results_distribution.few_results
    );
    println!();

    Ok(())
}

// ---------------------------------------------------------------------------
// Output Helpers
// ---------------------------------------------------------------------------

/// Prints an info message.
pub fn print_info(msg: &str) {
    println!("{} {}", "info:".blue().bold(), msg);
}

/// Prints a success message.
pub fn print_success(msg: &str) {
    println!("{} {}", "ok:".green().bold(), msg);
}

/// Prints a warning message.
pub fn print_warning(msg: &str) {
    eprintln!("{} {}", "warn:".yellow().bold(), msg);
}

/// Prints an error message.
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}

/// Prints an event message (for watch mode).
pub fn print_event(event_type: &str, path: &str) {
    let colored_type = match event_type {
        "CREATED" => event_type.green().bold(),
        "MODIFIED" => event_type.yellow().bold(),
        "DELETED" => event_type.red().bold(),
        "RENAMED" => event_type.cyan().bold(),
        _ => event_type.white().bold(),
    };
    println!("  {} {}", colored_type, path);
}

/// Prints search results in a formatted table.
pub fn print_results(results: &[SearchResult], elapsed: &Duration) {
    if results.is_empty() {
        print_warning("No results found.");
        return;
    }

    println!(
        "\n{} Found {} results in {:.2?}\n",
        "Results:".cyan().bold(),
        results.len(),
        elapsed
    );

    for (i, result) in results.iter().enumerate() {
        let entry = &result.entry;
        let size_str = format_size(entry.size);
        let ext_str = entry.extension.as_deref().unwrap_or("-").to_string();
        let modified_str = entry
            .modified
            .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "-".into());

        println!(
            "  {}. {} {} {} {} {}",
            format!("{}", i + 1).dimmed(),
            entry.path.display().to_string().white(),
            format!("[{}]", ext_str).dimmed(),
            size_str.yellow(),
            modified_str.dimmed(),
            format!("({:.0})", result.score).dimmed(),
        );
    }

    println!();
}

/// Prints search results as JSON.
pub fn print_json(results: &[SearchResult]) -> SeekrResult<()> {
    let json = serde_json::to_string_pretty(results)?;
    println!("{}", json);
    Ok(())
}

/// Prints search results as CSV.
pub fn print_csv(results: &[SearchResult]) {
    println!("path,name,extension,size,modified,score");
    for result in results {
        let entry = &result.entry;
        let modified = entry.modified.map(|d| d.to_rfc3339()).unwrap_or_default();
        println!(
            "{},{},{},{},{},{:.0}",
            entry.path.display(),
            entry.file_name,
            entry.extension.as_deref().unwrap_or(""),
            entry.size,
            modified,
            result.score,
        );
    }
}

/// Prints index statistics.
pub fn print_stats(stats: &IndexStats) {
    println!("\n{}", "Index Statistics:".cyan().bold());
    println!(
        "  {} {}",
        "Files:".white(),
        stats.total_files.to_string().green()
    );
    println!(
        "  {} {}",
        "Directories:".white(),
        stats.total_dirs.to_string().green()
    );
    println!(
        "  {} {}",
        "Total Size:".white(),
        format_size(stats.total_size).yellow()
    );
    println!("  {} {}", "Hidden Files:".white(), stats.hidden_files);
    println!(
        "  {} {}",
        "Unique Extensions:".white(),
        stats.unique_extensions
    );
    if let Some(ref last) = stats.last_indexed {
        println!(
            "  {} {}",
            "Last Indexed:".white(),
            last.format("%Y-%m-%d %H:%M:%S UTC")
        );
    }
    println!("  {} {}", "Root:".white(), stats.root_path.display());
    println!();
}

/// Prints application configuration.
pub fn print_config(config: &AppConfig) {
    println!("\n{}", "Configuration:".cyan().bold());
    println!(
        "  {} {}",
        "Search Root:".white(),
        config
            .search_root
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(not set)".into())
    );
    println!(
        "  {} {}",
        "Cache Enabled:".white(),
        config.cache_enabled.to_string().green()
    );
    println!(
        "  {} {}s",
        "Cache TTL:".white(),
        config.cache_ttl.to_string().green()
    );
    println!(
        "  {} {}",
        "Default Limit:".white(),
        config.default_limit.to_string().green()
    );
    println!(
        "  {} {}",
        "Color:".white(),
        config.color.to_string().green()
    );
    if let Some(ref db) = config.database_path {
        println!("  {} {}", "Database:".white(), db.display());
    }
    println!();
}

/// Exports search results to a file.
pub fn export_results(
    results: &[SearchResult],
    path: &str,
    format: &OutputFormat,
) -> SeekrResult<()> {
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)?,
        OutputFormat::Csv => {
            let mut w = Vec::new();
            writeln!(w, "path,name,extension,size,modified,score")?;
            for result in results {
                let entry = &result.entry;
                let modified = entry.modified.map(|d| d.to_rfc3339()).unwrap_or_default();
                writeln!(
                    w,
                    "{},{},{},{},{},{:.0}",
                    entry.path.display(),
                    entry.file_name,
                    entry.extension.as_deref().unwrap_or(""),
                    entry.size,
                    modified,
                    result.score,
                )?;
            }
            String::from_utf8(w)?
        }
        OutputFormat::Pretty => {
            let mut w = Vec::new();
            for result in results {
                writeln!(w, "{} ({:.0})", result.entry.path.display(), result.score)?;
            }
            String::from_utf8(w)?
        }
    };

    std::fs::write(path, content)?;
    Ok(())
}

/// Formats a byte size into a human-readable string.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
        assert_eq!(format_size(1099511627776), "1.0 TB");
    }

    #[test]
    fn test_format_size_boundaries() {
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1025), "1.0 KB");
        assert_eq!(format_size(1048575), "1024.0 KB");
        assert_eq!(format_size(1048577), "1.0 MB");
    }

    #[test]
    fn test_output_format_from_arg() {
        assert!(matches!(
            OutputFormat::from(OutputFormatArg::Pretty),
            OutputFormat::Pretty
        ));
        assert!(matches!(
            OutputFormat::from(OutputFormatArg::Json),
            OutputFormat::Json
        ));
        assert!(matches!(
            OutputFormat::from(OutputFormatArg::Csv),
            OutputFormat::Csv
        ));
    }

    #[test]
    fn test_cli_parse_version() {
        let cli = Cli::try_parse_from(["seekr", "version"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn test_cli_parse_search() {
        let cli = Cli::try_parse_from(["seekr", "search", "hello"]);
        assert!(cli.is_ok());
        match cli.unwrap().command {
            Commands::Search { pattern, .. } => assert_eq!(pattern, "hello"),
            _ => panic!("expected Search command"),
        }
    }

    #[test]
    fn test_cli_parse_index() {
        let cli = Cli::try_parse_from(["seekr", "index", "/tmp"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn test_cli_parse_search_with_flags() {
        let cli = Cli::try_parse_from([
            "seekr",
            "search",
            "query",
            "--case-sensitive",
            "--regex",
            "--extension",
            "rs",
            "--limit",
            "10",
        ]);
        assert!(cli.is_ok());
    }
}
