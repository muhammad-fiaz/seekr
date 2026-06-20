//! Demonstrates the plugin system.

use seekr::error::SeekrResult;
use seekr::plugin::{Hook, HookResult, Plugin, PluginManager};

struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "logger"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "Logs search and index events"
    }

    fn handle_hook(&self, hook: Hook) -> SeekrResult<HookResult> {
        match hook {
            Hook::PreSearch(ref query) => println!("[Logger] Searching: {}", query.pattern),
            Hook::PostSearch(ref results) => println!("[Logger] Found {} results", results.len()),
            Hook::PreIndex(ref path) => println!("[Logger] Indexing: {}", path.display()),
            Hook::PostIndex(count) => println!("[Logger] Indexed {} entries", count),
            _ => {}
        }
        Ok(HookResult::Continue)
    }
}

fn main() {
    let mut manager = PluginManager::new();
    manager.register(Box::new(LoggerPlugin)).unwrap();

    println!("Registered plugins: {:?}", manager.list_plugins());

    manager
        .execute_hook(&Hook::PreSearch(seekr::types::SearchQuery {
            pattern: "test".into(),
            ..seekr::types::SearchQuery::default()
        }))
        .unwrap();
    manager.execute_hook(&Hook::PostSearch(vec![])).unwrap();
}
