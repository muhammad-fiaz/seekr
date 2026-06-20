use std::path::{Path, PathBuf};

use crate::error::{SeekrError, SeekrResult};
use crate::platform;
use crate::types::AppConfig;

/// The default configuration file name.
const CONFIG_FILE_NAME: &str = "seekr.toml";

/// Loads configuration from the default location or a specified path.
///
/// Priority order:
/// 1. Explicit path argument
/// 2. `SEEKR_CONFIG` environment variable
/// 3. Platform config directory (`~/.config/seekr/seekr.toml`)
/// 4. Current directory (`./seekr.toml`)
/// 5. Built-in defaults
pub fn load_config(explicit_path: Option<&Path>) -> SeekrResult<AppConfig> {
    let config_path = if let Some(path) = explicit_path {
        Some(path.to_path_buf())
    } else if let Ok(env_path) = std::env::var("SEEKR_CONFIG") {
        Some(PathBuf::from(env_path))
    } else if let Some(config_dir) = platform::config_dir() {
        let path = config_dir.join(CONFIG_FILE_NAME);
        if path.exists() { Some(path) } else { None }
    } else {
        None
    };

    let mut config = if let Some(path) = config_path {
        load_from_file(&path)?
    } else {
        AppConfig::default()
    };

    apply_env_overrides(&mut config);
    config.database_path = Some(platform::default_database_path()?);

    Ok(config)
}

/// Loads configuration from a specific TOML file.
fn load_from_file(path: &Path) -> SeekrResult<AppConfig> {
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let content = std::fs::read_to_string(path).map_err(|e| {
        SeekrError::Config(format!(
            "failed to read config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let config: AppConfig = toml::from_str(&content).map_err(|e| {
        SeekrError::Config(format!(
            "failed to parse config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    Ok(config)
}

/// Applies environment variable overrides to the configuration.
fn apply_env_overrides(config: &mut AppConfig) {
    if let Ok(val) = std::env::var("SEEKR_SEARCH_ROOT") {
        config.search_root = Some(PathBuf::from(val));
    }
    if let Ok(val) = std::env::var("SEEKR_CACHE_TTL") {
        if let Ok(ttl) = val.parse() {
            config.cache_ttl = ttl;
        }
    }
    if let Ok(val) = std::env::var("SEEKR_DEFAULT_LIMIT") {
        if let Ok(limit) = val.parse() {
            config.default_limit = limit;
        }
    }
    if let Ok(val) = std::env::var("SEEKR_COLOR") {
        config.color = val.parse().unwrap_or(config.color);
    }
    if let Ok(val) = std::env::var("SEEKR_CACHE_ENABLED") {
        config.cache_enabled = val.parse().unwrap_or(config.cache_enabled);
    }
}

/// Saves the configuration to the platform config directory.
pub fn save_config(config: &AppConfig) -> SeekrResult<PathBuf> {
    let config_dir = platform::config_dir()
        .ok_or_else(|| SeekrError::Config("could not determine config directory".into()))?;

    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join(CONFIG_FILE_NAME);
    let content = toml::to_string_pretty(config)
        .map_err(|e| SeekrError::Config(format!("failed to serialize config: {}", e)))?;

    std::fs::write(&path, &content)
        .map_err(|e| SeekrError::Config(format!("failed to write config file: {}", e)))?;

    Ok(path)
}

/// Generates a default configuration file content.
pub fn default_config_content() -> String {
    let config = AppConfig::default();
    toml::to_string_pretty(&config).unwrap_or_default()
}

/// Validates the configuration values.
pub fn validate_config(config: &AppConfig) -> SeekrResult<()> {
    if let Some(ref root) = config.search_root {
        if !root.exists() {
            return Err(SeekrError::Config(format!(
                "search root does not exist: {}",
                root.display()
            )));
        }
    }

    if config.cache_ttl == 0 {
        return Err(SeekrError::Config(
            "cache_ttl must be greater than 0".into(),
        ));
    }

    if config.default_limit == 0 {
        return Err(SeekrError::Config(
            "default_limit must be greater than 0".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(config.search_root.is_none());
        assert!(config.cache_enabled);
        assert_eq!(config.cache_ttl, 3600);
        assert_eq!(config.default_limit, 50);
    }

    #[test]
    fn test_validate_config() {
        let config = AppConfig::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_default_config_content() {
        let content = default_config_content();
        assert!(content.contains("cache_enabled"));
        assert!(content.contains("cache_ttl"));
    }
}
