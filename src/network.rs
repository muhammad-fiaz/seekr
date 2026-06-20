use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::{SeekrError, SeekrResult};
use crate::types::FileEntry;

/// Configuration for a network share.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkShare {
    /// Display name for this share.
    pub name: String,
    /// Network path (e.g., "\\\\server\\share" or "smb://server/share").
    pub path: String,
    /// Authentication credentials (optional).
    pub credentials: Option<NetworkCredentials>,
    /// Connection timeout.
    pub timeout: Duration,
    /// Whether this share is currently connected.
    pub connected: bool,
}

/// Authentication credentials for network shares.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCredentials {
    /// Username for authentication.
    pub username: String,
    /// Password (should be encrypted in production).
    pub password: String,
    /// Optional domain.
    pub domain: Option<String>,
}

/// Result of a network file listing.
#[derive(Debug, Clone)]
pub struct NetworkFileEntry {
    /// The file entry.
    pub entry: FileEntry,
    /// Whether this file is accessible.
    pub accessible: bool,
    /// Network latency for this file operation (if measured).
    pub latency: Option<Duration>,
}

/// Manages network file search operations.
pub struct NetworkManager {
    shares: Vec<NetworkShare>,
    default_timeout: Duration,
}

impl NetworkManager {
    /// Creates a new network manager.
    pub fn new() -> Self {
        Self {
            shares: Vec::new(),
            default_timeout: Duration::from_secs(30),
        }
    }

    /// Adds a network share.
    pub fn add_share(&mut self, share: NetworkShare) {
        tracing::info!("added network share: {} ({})", share.name, share.path);
        self.shares.push(share);
    }

    /// Removes a network share by name.
    pub fn remove_share(&mut self, name: &str) -> bool {
        let before = self.shares.len();
        self.shares.retain(|s| s.name != name);
        self.shares.len() < before
    }

    /// Lists all configured network shares.
    pub fn list_shares(&self) -> &[NetworkShare] {
        &self.shares
    }

    /// Tests connectivity to a network share.
    pub fn test_connection(&self, share_name: &str) -> SeekrResult<bool> {
        let share = self
            .shares
            .iter()
            .find(|s| s.name == share_name)
            .ok_or_else(|| {
                SeekrError::Index(format!("network share '{}' not found", share_name))
            })?;

        let path = Path::new(&share.path);
        let accessible = path.exists();

        tracing::info!(
            "connection test for '{}': {}",
            share_name,
            if accessible { "ok" } else { "failed" }
        );

        Ok(accessible)
    }

    /// Lists files in a network share directory.
    pub fn list_files(&self, share_name: &str, path: &Path) -> SeekrResult<Vec<FileEntry>> {
        let share = self
            .shares
            .iter()
            .find(|s| s.name == share_name)
            .ok_or_else(|| {
                SeekrError::Index(format!("network share '{}' not found", share_name))
            })?;

        let full_path = PathBuf::from(&share.path).join(path);

        if !full_path.exists() {
            return Err(SeekrError::PathNotFound(full_path));
        }

        let mut entries = Vec::new();
        for entry in std::fs::read_dir(&full_path).map_err(|e| {
            SeekrError::Io(std::io::Error::other(format!("failed to read dir: {}", e)))
        })? {
            let entry = entry.map_err(|e| {
                SeekrError::Io(std::io::Error::other(format!(
                    "failed to read entry: {}",
                    e
                )))
            })?;
            let metadata = entry.metadata().map_err(|e| {
                SeekrError::Io(std::io::Error::other(format!(
                    "failed to read metadata: {}",
                    e
                )))
            })?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            let is_hidden = file_name.starts_with('.');
            let is_dir = metadata.is_dir();
            let extension = if is_dir {
                None
            } else {
                entry
                    .path()
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
            };

            entries.push(FileEntry {
                id: None,
                path: entry.path(),
                file_name,
                extension,
                parent_dir: full_path.clone(),
                size: metadata.len(),
                modified: metadata.modified().ok().map(|t| t.into()),
                accessed: metadata.accessed().ok().map(|t| t.into()),
                is_hidden,
                is_dir,
                hash: None,
            });
        }

        Ok(entries)
    }

    /// Sets the default timeout for network operations.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Returns the default timeout.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list_shares() {
        let mut manager = NetworkManager::new();
        manager.add_share(NetworkShare {
            name: "share1".into(),
            path: "/tmp/share".into(),
            credentials: None,
            timeout: Duration::from_secs(30),
            connected: false,
        });
        assert_eq!(manager.list_shares().len(), 1);
    }

    #[test]
    fn test_remove_share() {
        let mut manager = NetworkManager::new();
        manager.add_share(NetworkShare {
            name: "share1".into(),
            path: "/tmp/share".into(),
            credentials: None,
            timeout: Duration::from_secs(30),
            connected: false,
        });
        assert!(manager.remove_share("share1"));
        assert!(!manager.remove_share("nonexistent"));
    }

    #[test]
    fn test_default_timeout() {
        let manager = NetworkManager::new();
        assert_eq!(manager.default_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_set_default_timeout() {
        let mut manager = NetworkManager::new();
        manager.set_default_timeout(Duration::from_secs(60));
        assert_eq!(manager.default_timeout(), Duration::from_secs(60));
    }

    #[test]
    fn test_test_connection_not_found() {
        let manager = NetworkManager::new();
        assert!(manager.test_connection("nonexistent").is_err());
    }
}
