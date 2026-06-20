use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::SeekrResult;
use crate::types::{SearchQuery, SearchResult};

/// A collaborator connected to a shared session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    /// Unique identifier for this collaborator.
    pub id: String,
    /// Display name.
    pub name: String,
    /// When they joined the session.
    pub joined_at: DateTime<Utc>,
    /// Last activity timestamp (epoch seconds).
    pub last_active: Option<i64>,
    /// Whether they are currently online.
    pub online: bool,
}

/// A shared search session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSession {
    /// Unique session identifier.
    pub id: String,
    /// Session name/title.
    pub name: String,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// Connected collaborators.
    pub collaborators: Vec<Collaborator>,
    /// Shared search query.
    pub query: SearchQuery,
    /// Shared results.
    pub results: Vec<SearchResult>,
    /// Shared bookmarks.
    pub bookmarks: Vec<Bookmark>,
}

/// A bookmarked search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Unique identifier.
    pub id: String,
    /// Bookmark title.
    pub title: String,
    /// The search result path.
    pub path: String,
    /// Optional note.
    pub note: Option<String>,
    /// Who created the bookmark.
    pub created_by: String,
    /// When it was created.
    pub created_at: DateTime<Utc>,
}

/// Manages collaborative search sessions.
pub struct CollaborationManager {
    sessions: Arc<Mutex<Vec<SearchSession>>>,
}

impl CollaborationManager {
    /// Creates a new collaboration manager.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a new shared search session.
    pub fn create_session(
        &self,
        name: &str,
        creator: &str,
        query: SearchQuery,
    ) -> SeekrResult<SearchSession> {
        let session = SearchSession {
            id: uuid_simple(),
            name: name.to_string(),
            created_at: Utc::now(),
            collaborators: vec![Collaborator {
                id: uuid_simple(),
                name: creator.to_string(),
                joined_at: Utc::now(),
                last_active: Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                ),
                online: true,
            }],
            query,
            results: Vec::new(),
            bookmarks: Vec::new(),
        };

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        sessions.push(session.clone());

        tracing::info!("created collaboration session: {}", session.name);
        Ok(session)
    }

    /// Joins an existing session.
    pub fn join_session(&self, session_id: &str, user_name: &str) -> SeekrResult<Collaborator> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        let session = sessions
            .iter_mut()
            .find(|s| s.id == session_id)
            .ok_or_else(|| {
                crate::error::SeekrError::Index(format!("session '{}' not found", session_id))
            })?;

        let collaborator = Collaborator {
            id: uuid_simple(),
            name: user_name.to_string(),
            joined_at: Utc::now(),
            last_active: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            ),
            online: true,
        };

        session.collaborators.push(collaborator.clone());
        tracing::info!("{} joined session '{}'", user_name, session.name);
        Ok(collaborator)
    }

    /// Updates the shared search results.
    pub fn update_results(&self, session_id: &str, results: Vec<SearchResult>) -> SeekrResult<()> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        if let Some(session) = sessions.iter_mut().find(|s| s.id == session_id) {
            session.results = results;
        }

        Ok(())
    }

    /// Adds a bookmark to a session.
    pub fn add_bookmark(
        &self,
        session_id: &str,
        title: &str,
        path: &str,
        created_by: &str,
        note: Option<String>,
    ) -> SeekrResult<Bookmark> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        let session = sessions
            .iter_mut()
            .find(|s| s.id == session_id)
            .ok_or_else(|| {
                crate::error::SeekrError::Index(format!("session '{}' not found", session_id))
            })?;

        let bookmark = Bookmark {
            id: uuid_simple(),
            title: title.to_string(),
            path: path.to_string(),
            note,
            created_by: created_by.to_string(),
            created_at: Utc::now(),
        };

        session.bookmarks.push(bookmark.clone());
        Ok(bookmark)
    }

    /// Lists all active sessions.
    pub fn list_sessions(&self) -> SeekrResult<Vec<SearchSession>> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        Ok(sessions.clone())
    }

    /// Removes a session.
    pub fn remove_session(&self, session_id: &str) -> SeekrResult<bool> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        let before = sessions.len();
        sessions.retain(|s| s.id != session_id);
        Ok(sessions.len() < before)
    }
}

impl Default for CollaborationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple UUID-like generator (no external dependency needed).
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let manager = CollaborationManager::new();
        let query = SearchQuery {
            pattern: "test".into(),
            ..SearchQuery::default()
        };
        let session = manager
            .create_session("test-session", "alice", query)
            .unwrap();
        assert_eq!(session.name, "test-session");
        assert_eq!(session.collaborators.len(), 1);
        assert_eq!(session.collaborators[0].name, "alice");
    }

    #[test]
    fn test_join_session() {
        let manager = CollaborationManager::new();
        let query = SearchQuery {
            pattern: "test".into(),
            ..SearchQuery::default()
        };
        let session = manager.create_session("test", "alice", query).unwrap();
        let collab = manager.join_session(&session.id, "bob").unwrap();
        assert_eq!(collab.name, "bob");
    }

    #[test]
    fn test_join_nonexistent_session() {
        let manager = CollaborationManager::new();
        assert!(manager.join_session("nonexistent", "bob").is_err());
    }

    #[test]
    fn test_list_sessions() {
        let manager = CollaborationManager::new();
        let query = SearchQuery::default();
        manager
            .create_session("s1", "alice", query.clone())
            .unwrap();
        manager.create_session("s2", "bob", query).unwrap();
        assert_eq!(manager.list_sessions().unwrap().len(), 2);
    }

    #[test]
    fn test_remove_session() {
        let manager = CollaborationManager::new();
        let query = SearchQuery::default();
        let session = manager.create_session("test", "alice", query).unwrap();
        assert!(manager.remove_session(&session.id).unwrap());
        assert!(!manager.remove_session("nonexistent").unwrap());
    }

    #[test]
    fn test_add_bookmark() {
        let manager = CollaborationManager::new();
        let query = SearchQuery::default();
        let session = manager.create_session("test", "alice", query).unwrap();
        let bookmark = manager
            .add_bookmark(
                &session.id,
                "important",
                "/path/to/file",
                "alice",
                Some("note".into()),
            )
            .unwrap();
        assert_eq!(bookmark.title, "important");
        assert_eq!(bookmark.path, "/path/to/file");
    }
}
