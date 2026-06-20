use std::path::PathBuf;

use crate::error::SeekrResult;
use crate::types::{FileEntry, SearchQuery, SearchResult};

/// Lifecycle hooks that plugins can implement.
#[derive(Clone)]
pub enum Hook {
    /// Called before a search is executed.
    PreSearch(SearchQuery),
    /// Called after a search is executed.
    PostSearch(Vec<SearchResult>),
    /// Called before indexing starts.
    PreIndex(PathBuf),
    /// Called after indexing completes.
    PostIndex(u64),
    /// Called when a file is encountered during indexing.
    OnFileIndexed(FileEntry),
    /// Called when a file is modified.
    OnFileModified(PathBuf),
}

/// The result of a hook execution.
pub enum HookResult {
    /// Continue with default behavior.
    Continue,
    /// Modify the search query before execution.
    ModifyQuery(SearchQuery),
    /// Replace the search results.
    ReplaceResults(Vec<SearchResult>),
    /// Skip this item (for file hooks).
    Skip,
    /// Stop plugin execution for this hook.
    Stop,
}

/// A trait that plugins must implement.
pub trait Plugin: Send + Sync {
    /// Returns the name of the plugin.
    fn name(&self) -> &str;

    /// Returns the version of the plugin.
    fn version(&self) -> &str;

    /// Returns a description of the plugin.
    fn description(&self) -> &str;

    /// Called when the plugin is registered.
    fn on_register(&mut self) -> SeekrResult<()> {
        Ok(())
    }

    /// Called when the plugin is unregistered.
    fn on_unregister(&mut self) -> SeekrResult<()> {
        Ok(())
    }

    /// Handles a hook invocation.
    fn handle_hook(&self, hook: Hook) -> SeekrResult<HookResult>;
}

/// Manages plugin registration and execution.
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// Creates a new empty plugin manager.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Registers a plugin.
    pub fn register(&mut self, mut plugin: Box<dyn Plugin>) -> SeekrResult<()> {
        plugin.on_register()?;
        tracing::info!("registered plugin: {} v{}", plugin.name(), plugin.version());
        self.plugins.push(plugin);
        Ok(())
    }

    /// Unregisters a plugin by name.
    pub fn unregister(&mut self, name: &str) -> SeekrResult<bool> {
        if let Some(pos) = self.plugins.iter().position(|p| p.name() == name) {
            let mut plugin = self.plugins.remove(pos);
            plugin.on_unregister()?;
            tracing::info!("unregistered plugin: {}", name);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns a list of registered plugin names.
    pub fn list_plugins(&self) -> Vec<(&str, &str, &str)> {
        self.plugins
            .iter()
            .map(|p| (p.name(), p.version(), p.description()))
            .collect()
    }

    /// Executes a hook across all registered plugins.
    pub fn execute_hook(&self, hook: &Hook) -> SeekrResult<HookResult> {
        for plugin in &self.plugins {
            match plugin.handle_hook(hook.clone())? {
                HookResult::Continue => {}
                other => return Ok(other),
            }
        }
        Ok(HookResult::Continue)
    }

    /// Returns the number of registered plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple example plugin that logs hook invocations.
pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn name(&self) -> &str {
        "logging"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "Logs all hook invocations for debugging"
    }

    fn handle_hook(&self, hook: Hook) -> SeekrResult<HookResult> {
        match &hook {
            Hook::PreSearch(query) => {
                tracing::debug!("[logging-plugin] pre-search: {}", query.pattern);
            }
            Hook::PostSearch(results) => {
                tracing::debug!("[logging-plugin] post-search: {} results", results.len());
            }
            Hook::PreIndex(path) => {
                tracing::debug!("[logging-plugin] pre-index: {}", path.display());
            }
            Hook::PostIndex(count) => {
                tracing::debug!("[logging-plugin] post-index: {} entries", count);
            }
            Hook::OnFileIndexed(entry) => {
                tracing::trace!("[logging-plugin] indexed: {}", entry.path.display());
            }
            Hook::OnFileModified(path) => {
                tracing::debug!("[logging-plugin] modified: {}", path.display());
            }
        }
        Ok(HookResult::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestPlugin {
        name: String,
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            &self.name
        }
        fn version(&self) -> &str {
            "0.1.0"
        }
        fn description(&self) -> &str {
            "Test plugin"
        }
        fn handle_hook(&self, _hook: Hook) -> SeekrResult<HookResult> {
            Ok(HookResult::Continue)
        }
    }

    #[test]
    fn test_register_and_list() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(TestPlugin {
            name: "test".into(),
        });
        manager.register(plugin).unwrap();
        assert_eq!(manager.count(), 1);
        let list = manager.list_plugins();
        assert_eq!(list[0].0, "test");
    }

    #[test]
    fn test_unregister() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(TestPlugin {
            name: "test".into(),
        });
        manager.register(plugin).unwrap();
        assert!(manager.unregister("test").unwrap());
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_unregister_nonexistent() {
        let mut manager = PluginManager::new();
        assert!(!manager.unregister("nonexistent").unwrap());
    }

    #[test]
    fn test_execute_hook_continue() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(TestPlugin {
            name: "test".into(),
        });
        manager.register(plugin).unwrap();
        let hook = Hook::PreSearch(SearchQuery::default());
        let result = manager.execute_hook(&hook).unwrap();
        assert!(matches!(result, HookResult::Continue));
    }

    #[test]
    fn test_logging_plugin() {
        let plugin = LoggingPlugin;
        assert_eq!(plugin.name(), "logging");
        assert_eq!(plugin.version(), "0.1.0");
        let hook = Hook::PreIndex(PathBuf::from("/tmp"));
        let result = plugin.handle_hook(hook).unwrap();
        assert!(matches!(result, HookResult::Continue));
    }
}
