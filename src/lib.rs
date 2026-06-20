//! Seekr - A fast, local-first, privacy-focused file search engine.
//!
//! Seekr is a modular, library-first file search engine written in Rust.
//! All functionality is exposed through a clean, reusable API. The CLI is
//! only an interface layer on top of the library.
//!
//! # Architecture
//!
//! The library is organized into the following modules:
//!
//! - [`types`] - Shared domain models, structures, and enums.
//! - [`error`] - Unified error types.
//! - [`database`] - SQLite-backed persistence layer.
//! - [`indexer`] - Filesystem traversal and metadata extraction.
//! - [`search`] - Query engine with regex, fuzzy, and filter support.
//! - [`watcher`] - Filesystem monitoring and event processing.
//! - [`platform`] - OS-specific abstractions.
//! - [`config`] - Configuration management.
//! - [`cache`] - Multi-layer caching system.
//! - [`core`] - Application facade coordinating all services.
//! - [`cli`] - Command-line interface.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use seekr::core::SeekrApp;
//! use seekr::types::SearchQuery;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = SeekrApp::default_config()?;
//!
//!     // Index the current directory
//!     app.index(std::path::Path::new("."))?;
//!
//!     // Search for files
//!     let query = SearchQuery {
//!         pattern: "main.rs".into(),
//!         ..SearchQuery::default()
//!     };
//!     let results = app.search(&query)?;
//!     for result in &results {
//!         println!("{}", result.entry.path.display());
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod cache;
pub mod cli;
pub mod config;
pub mod core;
pub mod database;
pub mod error;
pub mod indexer;
pub mod platform;
pub mod search;
pub mod types;
pub mod watcher;

pub use error::{SeekrError, SeekrResult};
pub use types::*;
