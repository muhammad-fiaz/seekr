use serde::{Deserialize, Serialize};

use crate::types::{FileEntry, SearchQuery, SearchResult};

/// Mobile API response wrapper.
#[derive(Debug, Serialize)]
pub struct MobileResponse<T: Serialize> {
    /// Whether the request was successful.
    pub ok: bool,
    /// Response data.
    pub data: Option<T>,
    /// Error message if unsuccessful.
    pub error: Option<String>,
    /// Response metadata.
    pub meta: ResponseMeta,
}

/// Response metadata for mobile clients.
#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    /// Total number of results.
    pub total: usize,
    /// Current page.
    pub page: usize,
    /// Page size.
    pub page_size: usize,
    /// Whether there are more pages.
    pub has_more: bool,
}

impl<T: Serialize> MobileResponse<T> {
    /// Creates a successful response.
    pub fn ok(data: T, total: usize, page: usize, page_size: usize) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            meta: ResponseMeta {
                total,
                page,
                page_size,
                has_more: (page + 1) * page_size < total,
            },
        }
    }

    /// Creates an error response.
    pub fn err(msg: &str, total: usize, page: usize, page_size: usize) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(msg.to_string()),
            meta: ResponseMeta {
                total,
                page,
                page_size,
                has_more: false,
            },
        }
    }
}

/// Lightweight file entry for mobile.
#[derive(Debug, Serialize)]
pub struct MobileFileEntry {
    /// File name.
    pub name: String,
    /// File path.
    pub path: String,
    /// File extension.
    pub extension: Option<String>,
    /// File size.
    pub size: u64,
    /// Size as human-readable string.
    pub size_str: String,
    /// Last modified.
    pub modified: Option<String>,
    /// Whether it's a directory.
    pub is_dir: bool,
}

impl From<&FileEntry> for MobileFileEntry {
    fn from(entry: &FileEntry) -> Self {
        Self {
            name: entry.file_name.clone(),
            path: entry.path.to_string_lossy().to_string(),
            extension: entry.extension.clone(),
            size: entry.size,
            size_str: format_size(entry.size),
            modified: entry
                .modified
                .map(|d| d.format("%Y-%m-%d %H:%M").to_string()),
            is_dir: entry.is_dir,
        }
    }
}

/// Mobile search request.
#[derive(Debug, Deserialize)]
pub struct MobileSearchRequest {
    /// Search pattern.
    pub q: String,
    /// Page number (0-indexed).
    #[serde(default)]
    pub page: usize,
    /// Page size.
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    /// Filter by extension.
    pub ext: Option<String>,
    /// Case-sensitive search.
    #[serde(default)]
    pub case_sensitive: bool,
}

fn default_page_size() -> usize {
    20
}

impl From<MobileSearchRequest> for SearchQuery {
    fn from(req: MobileSearchRequest) -> Self {
        SearchQuery {
            pattern: req.q,
            case_sensitive: req.case_sensitive,
            extension: req.ext,
            limit: Some(req.page_size),
            offset: req.page * req.page_size,
            ..SearchQuery::default()
        }
    }
}

/// Formats a byte size into a human-readable string.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Creates a mobile-optimized search response.
pub fn mobile_search(
    results: &[SearchResult],
    page: usize,
    page_size: usize,
) -> MobileResponse<Vec<MobileFileEntry>> {
    let total = results.len();
    let start = page * page_size;
    let end = (start + page_size).min(total);

    let entries: Vec<MobileFileEntry> = results[start..end]
        .iter()
        .map(|r| MobileFileEntry::from(&r.entry))
        .collect();

    MobileResponse::ok(entries, total, page, page_size)
}

/// Creates a mobile-optimized file list response.
pub fn mobile_files(
    entries: &[FileEntry],
    page: usize,
    page_size: usize,
) -> MobileResponse<Vec<MobileFileEntry>> {
    let total = entries.len();
    let start = page * page_size;
    let end = (start + page_size).min(total);

    let mobile_entries: Vec<MobileFileEntry> = entries[start..end]
        .iter()
        .map(MobileFileEntry::from)
        .collect();

    MobileResponse::ok(mobile_entries, total, page, page_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_mobile_response_ok() {
        let resp: MobileResponse<String> = MobileResponse::ok("data".into(), 10, 0, 5);
        assert!(resp.ok);
        assert!(resp.data.is_some());
        assert_eq!(resp.meta.total, 10);
        assert!(resp.meta.has_more);
    }

    #[test]
    fn test_mobile_response_err() {
        let resp: MobileResponse<String> = MobileResponse::err("fail", 0, 0, 20);
        assert!(!resp.ok);
        assert!(resp.error.is_some());
    }

    #[test]
    fn test_mobile_search_request_into_query() {
        let req = MobileSearchRequest {
            q: "test".into(),
            page: 1,
            page_size: 10,
            ext: Some("rs".into()),
            case_sensitive: true,
        };
        let query: SearchQuery = req.into();
        assert_eq!(query.pattern, "test");
        assert!(query.case_sensitive);
        assert_eq!(query.offset, 10);
    }
}
