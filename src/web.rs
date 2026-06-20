use serde::{Deserialize, Serialize};

use crate::core::SeekrApp;
use crate::types::{SearchQuery, SearchResult};

/// API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    /// Whether the request was successful.
    pub success: bool,
    /// Response data.
    pub data: Option<T>,
    /// Error message if unsuccessful.
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Creates a successful response.
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Creates an error response.
    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

/// Search request body for the API.
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    /// The search pattern.
    pub pattern: String,
    /// Whether the search is case-sensitive.
    #[serde(default)]
    pub case_sensitive: bool,
    /// Whether to use regex.
    #[serde(default)]
    pub use_regex: bool,
    /// Whether to use fuzzy matching.
    #[serde(default)]
    pub use_fuzzy: bool,
    /// Filter by extension.
    pub extension: Option<String>,
    /// Maximum number of results.
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

impl From<SearchRequest> for SearchQuery {
    fn from(req: SearchRequest) -> Self {
        SearchQuery {
            pattern: req.pattern,
            case_sensitive: req.case_sensitive,
            use_regex: req.use_regex,
            use_fuzzy: req.use_fuzzy,
            extension: req.extension,
            limit: Some(req.limit),
            ..SearchQuery::default()
        }
    }
}

/// Index request body.
#[derive(Debug, Deserialize)]
pub struct IndexRequest {
    /// The path to index.
    pub path: String,
    /// Whether to do incremental indexing.
    #[serde(default)]
    pub incremental: bool,
}

/// Statistics response.
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_files: u64,
    pub total_dirs: u64,
    pub total_size: u64,
    pub hidden_files: u64,
    pub unique_extensions: u64,
}

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

/// Web server configuration.
#[derive(Debug, Clone)]
pub struct WebConfig {
    /// Host to bind to.
    pub host: String,
    /// Port to listen on.
    pub port: u16,
    /// Whether to enable CORS.
    pub cors_enabled: bool,
    /// Allowed origins for CORS.
    pub allowed_origins: Vec<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            cors_enabled: true,
            allowed_origins: vec!["*".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_ok() {
        let resp: ApiResponse<String> = ApiResponse::ok("hello".into());
        assert!(resp.success);
        assert!(resp.data.is_some());
    }

    #[test]
    fn test_api_response_err() {
        let resp: ApiResponse<String> = ApiResponse::err("fail");
        assert!(!resp.success);
        assert!(resp.error.is_some());
    }

    #[test]
    fn test_health() {
        let resp = api_health();
        assert!(resp.success);
    }

    #[test]
    fn test_web_config_default() {
        let config = WebConfig::default();
        assert_eq!(config.port, 8080);
        assert!(config.cors_enabled);
    }

    #[test]
    fn test_search_request_into_query() {
        let req = SearchRequest {
            pattern: "test".into(),
            case_sensitive: true,
            use_regex: false,
            use_fuzzy: false,
            extension: Some("txt".into()),
            limit: 10,
        };
        let query: SearchQuery = req.into();
        assert_eq!(query.pattern, "test");
        assert!(query.case_sensitive);
        assert_eq!(query.extension, Some("txt".into()));
    }
}

/// Creates JSON API response for search.
pub fn api_search(app: &SeekrApp, req: SearchRequest) -> ApiResponse<Vec<SearchResult>> {
    let query: SearchQuery = req.into();
    match app.search(&query) {
        Ok(results) => ApiResponse::ok(results),
        Err(e) => ApiResponse::err(&format!("search failed: {}", e)),
    }
}

/// Creates JSON API response for index.
pub fn api_index(app: &SeekrApp, req: IndexRequest) -> ApiResponse<StatsResponse> {
    let path = std::path::Path::new(&req.path);
    match app.index(path) {
        Ok(stats) => ApiResponse::ok(StatsResponse {
            total_files: stats.total_files,
            total_dirs: stats.total_dirs,
            total_size: stats.total_size,
            hidden_files: stats.hidden_files,
            unique_extensions: stats.unique_extensions,
        }),
        Err(e) => ApiResponse::err(&format!("index failed: {}", e)),
    }
}

/// Creates JSON API response for stats.
pub fn api_stats(app: &SeekrApp, path: &str) -> ApiResponse<StatsResponse> {
    let p = std::path::Path::new(path);
    match app.stats(p) {
        Ok(stats) => ApiResponse::ok(StatsResponse {
            total_files: stats.total_files,
            total_dirs: stats.total_dirs,
            total_size: stats.total_size,
            hidden_files: stats.hidden_files,
            unique_extensions: stats.unique_extensions,
        }),
        Err(e) => ApiResponse::err(&format!("stats failed: {}", e)),
    }
}

/// Creates JSON API response for health check.
pub fn api_health() -> ApiResponse<HealthResponse> {
    ApiResponse::ok(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: 0,
    })
}
