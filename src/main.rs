use clap::Parser;

use seekr::cli::{Cli, dispatch};
use seekr::error::SeekrResult;

fn main() -> SeekrResult<()> {
    let cli = Cli::parse();

    let level = if cli.verbose { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level)),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    if cli.no_color {
        colored::control::set_override(false);
    }

    dispatch(&cli)
}
