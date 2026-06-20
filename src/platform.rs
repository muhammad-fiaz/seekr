use std::path::{Path, PathBuf};

use crate::error::{SeekrError, SeekrResult};

/// Returns the platform-specific configuration directory for Seekr.
pub fn config_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "muhammad-fiaz", "seekr")
        .map(|dirs| dirs.config_dir().to_path_buf())
}

/// Returns the platform-specific cache directory for Seekr.
pub fn cache_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "muhammad-fiaz", "seekr")
        .map(|dirs| dirs.cache_dir().to_path_buf())
}

/// Returns the platform-specific data directory for Seekr.
pub fn data_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "muhammad-fiaz", "seekr")
        .map(|dirs| dirs.data_dir().to_path_buf())
}

/// Returns the default database file path.
pub fn default_database_path() -> SeekrResult<PathBuf> {
    let dir = data_dir()
        .ok_or_else(|| SeekrError::Config("could not determine platform data directory".into()))?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("seekr.db"))
}

/// Opens a file with the system's default application.
pub fn open_file(path: &Path) -> SeekrResult<()> {
    if !path.exists() {
        return Err(SeekrError::PathNotFound(path.to_path_buf()));
    }
    open::that(path)
        .map_err(|e| SeekrError::Io(std::io::Error::other(format!("failed to open file: {}", e))))
}

/// Opens the containing directory of a file in the file manager.
pub fn open_containing_directory(path: &Path) -> SeekrResult<()> {
    let dir = if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent().unwrap_or(Path::new(".")).to_path_buf()
    };

    if !dir.exists() {
        return Err(SeekrError::PathNotFound(dir));
    }

    open::that(&dir).map_err(|e| {
        SeekrError::Io(std::io::Error::other(format!(
            "failed to open directory: {}",
            e
        )))
    })
}

/// Reveals a file in the platform's file manager (e.g., Finder on macOS, Explorer on Windows).
pub fn reveal_file(path: &Path) -> SeekrResult<()> {
    if !path.exists() {
        return Err(SeekrError::PathNotFound(path.to_path_buf()));
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| {
                SeekrError::Io(std::io::Error::other(format!(
                    "failed to reveal file: {}",
                    e
                )))
            })?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(format!("/select,{}", path.to_string_lossy()))
            .spawn()
            .map_err(|e| {
                SeekrError::Io(std::io::Error::other(format!(
                    "failed to reveal file: {}",
                    e
                )))
            })?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        let dir = path.parent().unwrap_or(Path::new("."));
        open::that(dir).map_err(|e| {
            SeekrError::Io(std::io::Error::other(format!(
                "failed to reveal file: {}",
                e
            )))
        })
    }
}

/// Normalizes a path for display, resolving `..` and `.` components.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            other => components.push(other),
        }
    }
    components.iter().collect()
}

/// Returns the home directory of the current user.
pub fn home_dir() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let dir = config_dir();
        assert!(dir.is_some());
    }

    #[test]
    fn test_cache_dir() {
        let dir = cache_dir();
        assert!(dir.is_some());
    }

    #[test]
    fn test_normalize_path() {
        let path = PathBuf::from("/home/user/../user/./Documents");
        let normalized = normalize_path(&path);
        assert_eq!(normalized, PathBuf::from("/home/user/Documents"));
    }
}
