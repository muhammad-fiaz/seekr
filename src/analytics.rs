use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{FileEntry, IndexStats};

/// Analytics data for a search operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAnalytics {
    /// Total number of searches performed.
    pub total_searches: u64,
    /// Average search duration in milliseconds.
    pub avg_search_duration_ms: f64,
    /// Most common search patterns.
    pub top_patterns: Vec<(String, u64)>,
    /// Search patterns by time of day.
    pub hourly_distribution: Vec<u64>,
    /// Search results distribution.
    pub results_distribution: ResultsDistribution,
}

/// Distribution of search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultsDistribution {
    /// Searches with 0 results.
    pub no_results: u64,
    /// Searches with 1-10 results.
    pub few_results: u64,
    /// Searches with 11-100 results.
    pub moderate_results: u64,
    /// Searches with 100+ results.
    pub many_results: u64,
}

/// File type analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeAnalytics {
    /// File count by extension.
    pub extension_counts: HashMap<String, u64>,
    /// Total size by extension.
    pub extension_sizes: HashMap<String, u64>,
    /// Most common extensions.
    pub top_extensions: Vec<(String, u64)>,
    /// Average file size by extension.
    pub avg_sizes: HashMap<String, f64>,
}

/// Comprehensive analytics report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    /// When this report was generated.
    pub generated_at: DateTime<Utc>,
    /// Index statistics.
    pub index_stats: IndexStats,
    /// Search analytics.
    pub search: SearchAnalytics,
    /// File type analytics.
    pub file_types: FileTypeAnalytics,
    /// Performance metrics.
    pub performance: PerformanceMetrics,
}

/// Performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average indexing speed (files per second).
    pub indexing_speed: f64,
    /// Average search speed (queries per second).
    pub search_speed: f64,
    /// Cache hit rate.
    pub cache_hit_rate: f64,
    /// Memory usage estimate in bytes.
    pub memory_usage: u64,
}

/// Collects and analyzes search data.
pub struct AnalyticsCollector {
    search_durations: Vec<f64>,
    search_patterns: HashMap<String, u64>,
    result_counts: Vec<usize>,
}

impl AnalyticsCollector {
    /// Creates a new analytics collector.
    pub fn new() -> Self {
        Self {
            search_durations: Vec::new(),
            search_patterns: HashMap::new(),
            result_counts: Vec::new(),
        }
    }

    /// Records a search operation.
    pub fn record_search(&mut self, pattern: &str, duration_ms: f64, result_count: usize) {
        self.search_durations.push(duration_ms);
        *self.search_patterns.entry(pattern.to_string()).or_insert(0) += 1;
        self.result_counts.push(result_count);
    }

    /// Generates search analytics.
    pub fn search_analytics(&self) -> SearchAnalytics {
        let total_searches = self.search_durations.len() as u64;
        let avg_search_duration_ms = if total_searches > 0 {
            self.search_durations.iter().sum::<f64>() / total_searches as f64
        } else {
            0.0
        };

        let mut top_patterns: Vec<(String, u64)> = self
            .search_patterns
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        top_patterns.sort_by_key(|b| std::cmp::Reverse(b.1));
        top_patterns.truncate(10);

        let hourly_distribution = vec![0u64; 24];

        let no_results = self.result_counts.iter().filter(|&&c| c == 0).count() as u64;
        let few_results = self
            .result_counts
            .iter()
            .filter(|&&c| c > 0 && c <= 10)
            .count() as u64;
        let moderate_results = self
            .result_counts
            .iter()
            .filter(|&&c| c > 10 && c <= 100)
            .count() as u64;
        let many_results = self.result_counts.iter().filter(|&&c| c > 100).count() as u64;

        SearchAnalytics {
            total_searches,
            avg_search_duration_ms,
            top_patterns,
            hourly_distribution,
            results_distribution: ResultsDistribution {
                no_results,
                few_results,
                moderate_results,
                many_results,
            },
        }
    }
}

impl Default for AnalyticsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_search() {
        let mut collector = AnalyticsCollector::new();
        collector.record_search("pattern", 50.0, 10);
        let analytics = collector.search_analytics();
        assert_eq!(analytics.total_searches, 1);
        assert_eq!(analytics.avg_search_duration_ms, 50.0);
    }

    #[test]
    fn test_search_analytics_empty() {
        let collector = AnalyticsCollector::new();
        let analytics = collector.search_analytics();
        assert_eq!(analytics.total_searches, 0);
    }

    #[test]
    fn test_top_patterns() {
        let mut collector = AnalyticsCollector::new();
        collector.record_search("common", 10.0, 5);
        collector.record_search("common", 10.0, 5);
        collector.record_search("rare", 5.0, 3);
        let analytics = collector.search_analytics();
        assert!(analytics.top_patterns.len() <= 10);
        assert_eq!(analytics.top_patterns[0].0, "common");
        assert_eq!(analytics.top_patterns[0].1, 2);
    }

    #[test]
    fn test_results_distribution() {
        let mut collector = AnalyticsCollector::new();
        collector.record_search("a", 10.0, 0);
        collector.record_search("b", 10.0, 5);
        collector.record_search("c", 10.0, 50);
        collector.record_search("d", 10.0, 200);
        let analytics = collector.search_analytics();
        assert_eq!(analytics.results_distribution.no_results, 1);
        assert_eq!(analytics.results_distribution.few_results, 1);
        assert_eq!(analytics.results_distribution.moderate_results, 1);
        assert_eq!(analytics.results_distribution.many_results, 1);
    }

    #[test]
    fn test_analyze_file_types() {
        let entries = vec![
            FileEntry {
                id: None,
                path: "/tmp/test.rs".into(),
                file_name: "test.rs".into(),
                extension: Some("rs".into()),
                parent_dir: "/tmp".into(),
                size: 100,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
            FileEntry {
                id: None,
                path: "/tmp/other.rs".into(),
                file_name: "other.rs".into(),
                extension: Some("rs".into()),
                parent_dir: "/tmp".into(),
                size: 200,
                modified: None,
                accessed: None,
                is_hidden: false,
                is_dir: false,
                hash: None,
            },
        ];
        let analytics = analyze_file_types(&entries);
        assert_eq!(*analytics.extension_counts.get("rs").unwrap(), 2);
        assert_eq!(*analytics.extension_sizes.get("rs").unwrap(), 300);
    }

    #[test]
    fn test_analyze_empty() {
        let analytics = analyze_file_types(&[]);
        assert!(analytics.top_extensions.is_empty());
    }
}

/// Analyzes file types from a list of entries.
pub fn analyze_file_types(entries: &[FileEntry]) -> FileTypeAnalytics {
    let mut extension_counts: HashMap<String, u64> = HashMap::new();
    let mut extension_sizes: HashMap<String, u64> = HashMap::new();

    for entry in entries {
        if let Some(ref ext) = entry.extension {
            *extension_counts.entry(ext.clone()).or_insert(0) += 1;
            *extension_sizes.entry(ext.clone()).or_insert(0) += entry.size;
        }
    }

    let mut top_extensions: Vec<(String, u64)> = extension_counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    top_extensions.sort_by_key(|b| std::cmp::Reverse(b.1));
    top_extensions.truncate(10);

    let avg_sizes: HashMap<String, f64> = extension_counts
        .iter()
        .map(|(ext, &count)| {
            let total_size = extension_sizes.get(ext).copied().unwrap_or(0);
            (ext.clone(), total_size as f64 / count as f64)
        })
        .collect();

    FileTypeAnalytics {
        extension_counts,
        extension_sizes,
        top_extensions,
        avg_sizes,
    }
}

/// Generates a comprehensive analytics report.
pub fn generate_report(
    collector: &AnalyticsCollector,
    entries: &[FileEntry],
    stats: &IndexStats,
) -> AnalyticsReport {
    let search = collector.search_analytics();
    let file_types = analyze_file_types(entries);

    AnalyticsReport {
        generated_at: Utc::now(),
        index_stats: stats.clone(),
        search,
        file_types,
        performance: PerformanceMetrics {
            indexing_speed: 0.0,
            search_speed: 0.0,
            cache_hit_rate: 0.0,
            memory_usage: 0,
        },
    }
}
