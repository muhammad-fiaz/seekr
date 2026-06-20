//! # Seekr Platform Examples
//!
//! Demonstrates platform-specific file operations.

use seekr::platform;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Platform directories
    println!("=== Platform Directories ===");
    if let Some(config_dir) = platform::config_dir() {
        println!("Config dir: {}", config_dir.display());
    }
    if let Some(cache_dir) = platform::cache_dir() {
        println!("Cache dir: {}", cache_dir.display());
    }
    if let Some(data_dir) = platform::data_dir() {
        println!("Data dir: {}", data_dir.display());
    }
    if let Some(home) = platform::home_dir() {
        println!("Home dir: {}", home.display());
    }

    // 2. Default database path
    println!("\n=== Database Path ===");
    let db_path = platform::default_database_path()?;
    println!("Default DB: {}", db_path.display());

    // 3. Path normalization
    println!("\n=== Path Normalization ===");
    let messy = Path::new("/home/user/../user/./Documents/../../../tmp");
    let clean = platform::normalize_path(messy);
    println!("  {} -> {}", messy.display(), clean.display());

    // 4. File operations (these open GUI apps, commented out for safety)
    // platform::open_file(Path::new("Cargo.toml"))?;
    // platform::open_containing_directory(Path::new("src/main.rs"))?;
    // platform::reveal_file(Path::new("src/main.rs"))?;

    println!("\n=== Platform Info ===");
    println!("OS: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);

    Ok(())
}
