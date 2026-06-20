//! # Seekr Configuration Examples
//!
//! Demonstrates loading, saving, and using configuration.

use seekr::config;
use seekr::types::AppConfig;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Default configuration
    println!("=== Default Configuration ===");
    let config = AppConfig::default();
    println!("Cache enabled: {}", config.cache_enabled);
    println!("Cache TTL: {}s", config.cache_ttl);
    println!("Default limit: {}", config.default_limit);

    // 2. Generate config file content
    println!("\n=== Generated Config Content ===");
    let content = config::default_config_content();
    println!("{}", content);

    // 3. Validate configuration
    println!("=== Validation ===");
    match config::validate_config(&config) {
        Ok(()) => println!("Config is valid!"),
        Err(e) => println!("Validation error: {}", e),
    }

    // 4. Load config (will use defaults if no file exists)
    println!("\n=== Load Config ===");
    let loaded = config::load_config(None)?;
    println!(
        "Loaded config: cache={}, limit={}",
        loaded.cache_enabled, loaded.default_limit
    );

    // 5. Custom configuration
    println!("\n=== Custom Config ===");
    let custom = AppConfig {
        search_root: Some(Path::new(".").to_path_buf()),
        cache_enabled: false,
        cache_ttl: 600,
        default_limit: 100,
        color: true,
        database_path: None,
        indexer: seekr::types::IndexerConfig::default(),
    };
    config::validate_config(&custom)?;
    println!("Custom config valid!");

    Ok(())
}
