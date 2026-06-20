use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::error::{SeekrError, SeekrResult};
use crate::types::FileEvent;

/// Watches a directory for filesystem changes and sends events through a channel.
///
/// Returns a receiver for file events and a handle to the watcher.
/// The watcher is automatically stopped when the returned `WatcherHandle` is dropped.
pub fn watch_directory(
    config: crate::types::WatchConfig,
) -> SeekrResult<(std::sync::mpsc::Receiver<FileEvent>, WatcherHandle)> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |result: Result<Event, notify::Error>| {
            if let Ok(event) = result {
                let file_events = convert_event(event);
                for fe in file_events {
                    let _ = tx.send(fe);
                }
            }
        },
        notify::Config::default()
            .with_poll_interval(std::time::Duration::from_millis(config.debounce_ms)),
    )
    .map_err(|e| SeekrError::Watcher(format!("failed to create watcher: {}", e)))?;

    let mode = if config.recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher
        .watch(&config.path, mode)
        .map_err(|e| SeekrError::Watcher(format!("failed to watch path: {}", e)))?;

    Ok((rx, WatcherHandle { _watcher: watcher }))
}

/// Converts a `notify::Event` into one or more `FileEvent` values.
fn convert_event(event: Event) -> Vec<FileEvent> {
    use notify::event::ModifyKind;
    use notify::event::RenameMode;
    let mut events = Vec::new();

    match event.kind {
        EventKind::Create(_) => {
            for path in event.paths {
                events.push(FileEvent::Created(path));
            }
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
            if event.paths.len() >= 2 {
                events.push(FileEvent::Renamed {
                    from: event.paths[0].clone(),
                    to: event.paths[1].clone(),
                });
            }
        }
        EventKind::Modify(_) => {
            for path in event.paths {
                events.push(FileEvent::Modified(path));
            }
        }
        EventKind::Remove(_) => {
            for path in event.paths {
                events.push(FileEvent::Deleted(path));
            }
        }
        _ => {}
    }

    events
}

/// A handle to a running filesystem watcher.
///
/// When dropped, the watcher is automatically stopped.
pub struct WatcherHandle {
    _watcher: RecommendedWatcher,
}

impl WatcherHandle {
    /// Stops the watcher explicitly.
    pub fn stop(self) {
        drop(self);
    }
}

/// Watches a directory and applies a callback for each event.
///
/// This is a blocking function that runs until the watcher is dropped.
pub fn watch_and_process<F>(config: crate::types::WatchConfig, mut callback: F) -> SeekrResult<()>
where
    F: FnMut(FileEvent),
{
    let (rx, _watcher) = watch_directory(config)?;

    while let Ok(event) = rx.recv() {
        callback(event);
    }

    Ok(())
}

/// Resolves a `FileEvent` to check if the path still exists.
pub fn resolve_event(event: FileEvent) -> Option<FileEvent> {
    match &event {
        FileEvent::Created(path) | FileEvent::Modified(path) => {
            if path.exists() {
                Some(event)
            } else {
                None
            }
        }
        FileEvent::Deleted(_) => Some(event),
        FileEvent::Renamed { from: _, to } => {
            if to.exists() {
                Some(event)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::{Event, EventKind, event::CreateKind};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_convert_event() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/test.txt")],
            attrs: Default::default(),
        };
        let events = convert_event(event);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], FileEvent::Created(_)));
    }

    #[test]
    fn test_resolve_event_exists() {
        let dir = std::env::temp_dir().join("seekr_test_resolve");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("test.txt");
        let _ = fs::write(&file, "hello");

        let event = FileEvent::Created(file.clone());
        assert!(resolve_event(event).is_some());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_resolve_event_deleted() {
        let event = FileEvent::Deleted(PathBuf::from("/nonexistent/file.txt"));
        assert!(resolve_event(event).is_some());
    }

    #[test]
    fn test_stop_watcher() {
        let dir = std::env::temp_dir().join("seekr_test_watcher_stop");
        let _ = fs::create_dir_all(&dir);

        let config = crate::types::WatchConfig {
            path: dir.clone(),
            recursive: false,
            debounce_ms: 100,
        };

        let (_rx, handle) = watch_directory(config).unwrap();
        handle.stop();

        let _ = fs::remove_dir_all(&dir);
    }
}
