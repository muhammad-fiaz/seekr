use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::SeekrResult;

/// The status of a distributed node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Online,
    Offline,
    Syncing,
    Error(String),
}

/// Represents a node in the distributed indexing cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedNode {
    /// Unique identifier for this node.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Network address (e.g., "192.168.1.100:8080").
    pub address: String,
    /// Paths this node is responsible for indexing.
    pub paths: Vec<PathBuf>,
    /// Current status.
    pub status: NodeStatus,
    /// Last time the node was seen (epoch seconds).
    pub last_seen: Option<i64>,
    /// Number of indexed files on this node.
    pub indexed_count: u64,
    /// Node capabilities.
    pub capabilities: NodeCapabilities,
}

/// Capabilities of a distributed node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCapabilities {
    /// Whether this node can perform indexing.
    pub can_index: bool,
    /// Whether this node can perform searches.
    pub can_search: bool,
    /// Maximum number of files this node can index.
    pub max_files: Option<u64>,
    /// Supported file types.
    pub supported_types: Vec<String>,
}

/// Manages a cluster of distributed nodes.
pub struct DistributedManager {
    nodes: Arc<Mutex<Vec<DistributedNode>>>,
    sync_interval: Duration,
}

impl DistributedManager {
    /// Creates a new distributed manager.
    pub fn new(sync_interval: Duration) -> Self {
        Self {
            nodes: Arc::new(Mutex::new(Vec::new())),
            sync_interval,
        }
    }

    /// Adds a node to the cluster.
    pub fn add_node(&self, node: DistributedNode) -> SeekrResult<()> {
        let mut nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        if nodes.iter().any(|n| n.id == node.id) {
            return Err(crate::error::SeekrError::Index(format!(
                "node {} already exists",
                node.id
            )));
        }

        tracing::info!("added distributed node: {} ({})", node.name, node.address);
        nodes.push(node);
        Ok(())
    }

    /// Removes a node from the cluster.
    pub fn remove_node(&self, node_id: &str) -> SeekrResult<bool> {
        let mut nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        let before = nodes.len();
        nodes.retain(|n| n.id != node_id);
        Ok(nodes.len() < before)
    }

    /// Returns a list of all nodes in the cluster.
    pub fn list_nodes(&self) -> SeekrResult<Vec<DistributedNode>> {
        let nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        Ok(nodes.clone())
    }

    /// Returns the number of online nodes.
    pub fn online_count(&self) -> SeekrResult<usize> {
        let nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        Ok(nodes
            .iter()
            .filter(|n| n.status == NodeStatus::Online)
            .count())
    }

    /// Updates the status of a node.
    pub fn update_node_status(&self, node_id: &str, status: NodeStatus) -> SeekrResult<bool> {
        let mut nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;

        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
            node.status = status;
            node.last_seen = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the sync interval.
    pub fn sync_interval(&self) -> Duration {
        self.sync_interval
    }

    /// Returns total indexed count across all nodes.
    pub fn total_indexed(&self) -> SeekrResult<u64> {
        let nodes = self
            .nodes
            .lock()
            .map_err(|e| crate::error::SeekrError::Index(format!("lock error: {}", e)))?;
        Ok(nodes.iter().map(|n| n.indexed_count).sum())
    }
}

impl Default for DistributedManager {
    fn default() -> Self {
        Self::new(Duration::from_secs(300))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, name: &str) -> DistributedNode {
        DistributedNode {
            id: id.into(),
            name: name.into(),
            address: "127.0.0.1:8080".into(),
            paths: vec![],
            status: NodeStatus::Online,
            last_seen: None,
            indexed_count: 0,
            capabilities: NodeCapabilities::default(),
        }
    }

    #[test]
    fn test_add_and_list_nodes() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        manager.add_node(make_node("1", "node1")).unwrap();
        manager.add_node(make_node("2", "node2")).unwrap();
        let nodes = manager.list_nodes().unwrap();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_add_duplicate_node() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        manager.add_node(make_node("1", "node1")).unwrap();
        let result = manager.add_node(make_node("1", "node1"));
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_node() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        manager.add_node(make_node("1", "node1")).unwrap();
        assert!(manager.remove_node("1").unwrap());
        assert!(!manager.remove_node("nonexistent").unwrap());
        assert_eq!(manager.list_nodes().unwrap().len(), 0);
    }

    #[test]
    fn test_online_count() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        manager.add_node(make_node("1", "node1")).unwrap();
        manager.add_node(make_node("2", "node2")).unwrap();
        assert_eq!(manager.online_count().unwrap(), 2);
    }

    #[test]
    fn test_update_node_status() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        manager.add_node(make_node("1", "node1")).unwrap();
        assert!(
            manager
                .update_node_status("1", NodeStatus::Offline)
                .unwrap()
        );
        assert_eq!(manager.online_count().unwrap(), 0);
        assert!(
            !manager
                .update_node_status("nonexistent", NodeStatus::Online)
                .unwrap()
        );
    }

    #[test]
    fn test_total_indexed() {
        let manager = DistributedManager::new(Duration::from_secs(60));
        let mut n1 = make_node("1", "node1");
        n1.indexed_count = 100;
        let mut n2 = make_node("2", "node2");
        n2.indexed_count = 200;
        manager.add_node(n1).unwrap();
        manager.add_node(n2).unwrap();
        assert_eq!(manager.total_indexed().unwrap(), 300);
    }

    #[test]
    fn test_sync_interval() {
        let manager = DistributedManager::new(Duration::from_secs(120));
        assert_eq!(manager.sync_interval(), Duration::from_secs(120));
    }
}
