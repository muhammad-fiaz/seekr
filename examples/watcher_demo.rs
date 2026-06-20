//! # Seekr Watcher Example
//!
//! Demonstrates filesystem watching with event callbacks.
//! Note: This example runs until interrupted (Ctrl+C).

use seekr::types::{FileEvent, WatchConfig};
use seekr::watcher;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let watch_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    println!("Watching: {}", watch_path.display());
    println!("Try creating, modifying, or deleting files in this directory.");
    println!("Press Ctrl+C to stop.\n");

    let config = WatchConfig {
        path: watch_path,
        recursive: true,
        debounce_ms: 300,
    };

    watcher::watch_and_process(config, |event| match &event {
        FileEvent::Created(p) => println!("  [CREATED]  {}", p.display()),
        FileEvent::Modified(p) => println!("  [MODIFIED] {}", p.display()),
        FileEvent::Deleted(p) => println!("  [DELETED]  {}", p.display()),
        FileEvent::Renamed { from, to } => {
            println!("  [RENAMED]  {} -> {}", from.display(), to.display());
        }
    })?;

    Ok(())
}
