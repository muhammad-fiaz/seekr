use crate::types::{FileEntry, SearchQuery};

/// Features extracted from a file entry for ML scoring.
#[derive(Debug, Clone)]
pub struct FileFeatures {
    /// File name length.
    pub name_length: f64,
    /// File size (log-scaled).
    pub size_log: f64,
    /// Extension one-hot encoding (simplified).
    pub has_extension: f64,
    /// Is hidden file.
    pub is_hidden: f64,
    /// Days since last modification.
    pub days_since_modified: f64,
    /// Path depth.
    pub path_depth: f64,
    /// Name contains pattern (binary).
    pub name_match: f64,
    /// Path contains pattern (binary).
    pub path_match: f64,
}

/// A simple linear model for relevance scoring.
pub struct LinearRelevanceModel {
    /// Model weights.
    weights: Vec<f64>,
    /// Bias term.
    bias: f64,
}

impl LinearRelevanceModel {
    /// Creates a new model with default weights.
    pub fn new() -> Self {
        Self {
            weights: vec![
                0.3,  // name_length
                0.1,  // size_log
                0.2,  // has_extension
                -0.5, // is_hidden
                0.15, // days_since_modified (negative = newer preferred)
                0.05, // path_depth
                0.8,  // name_match
                0.5,  // path_match
            ],
            bias: -0.2,
        }
    }

    /// Creates a model with custom weights.
    pub fn with_weights(weights: Vec<f64>, bias: f64) -> Self {
        Self { weights, bias }
    }

    /// Extracts features from a file entry.
    pub fn extract_features(entry: &FileEntry, query: &SearchQuery) -> FileFeatures {
        let name = &entry.file_name;
        let path_str = entry.path.to_string_lossy();
        let pattern = &query.pattern;

        let name_match = if query.case_sensitive {
            name.contains(pattern) as i32 as f64
        } else {
            name.to_lowercase().contains(&pattern.to_lowercase()) as i32 as f64
        };

        let path_match = if query.case_sensitive {
            path_str.contains(pattern) as i32 as f64
        } else {
            path_str.to_lowercase().contains(&pattern.to_lowercase()) as i32 as f64
        };

        let days_since_modified = entry
            .modified
            .map(|m| {
                let now = chrono::Utc::now();
                now.signed_duration_since(m).num_days() as f64
            })
            .unwrap_or(365.0);

        let path_depth = entry.path.components().count() as f64;

        FileFeatures {
            name_length: name.len() as f64,
            size_log: (entry.size as f64 + 1.0).ln(),
            has_extension: entry.extension.is_some() as i32 as f64,
            is_hidden: entry.is_hidden as i32 as f64,
            days_since_modified,
            path_depth,
            name_match,
            path_match,
        }
    }

    /// Scores a file entry using the linear model.
    pub fn score(&self, features: &FileFeatures) -> f64 {
        let feature_values = [
            features.name_length,
            features.size_log,
            features.has_extension,
            features.is_hidden,
            features.days_since_modified,
            features.path_depth,
            features.name_match,
            features.path_match,
        ];

        let dot_product: f64 = feature_values
            .iter()
            .zip(self.weights.iter())
            .map(|(f, w)| f * w)
            .sum();

        (dot_product + self.bias).max(0.0)
    }

    /// Scores a file entry directly.
    pub fn score_entry(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let features = Self::extract_features(entry, query);
        self.score(&features)
    }

    /// Normalizes scores to 0-100 range.
    pub fn normalize_scores(scores: &mut [f64]) {
        if scores.is_empty() {
            return;
        }
        let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let range = max_score - min_score;

        if range > 0.0 {
            for score in scores.iter_mut() {
                *score = (*score - min_score) / range * 100.0;
            }
        }
    }

    /// Returns the model weights.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Returns the bias term.
    pub fn bias(&self) -> f64 {
        self.bias
    }
}

impl Default for LinearRelevanceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(name: &str, size: u64) -> FileEntry {
        FileEntry {
            id: None,
            path: format!("/tmp/{}", name).into(),
            file_name: name.into(),
            size,
            extension: name.split('.').last().map(|s| s.to_string()),
            is_dir: false,
            is_hidden: false,
            modified: None,
            parent_dir: "/tmp".into(),
            accessed: None,
            hash: None,
        }
    }

    #[test]
    fn test_extract_features() {
        let entry = make_entry("test.txt", 500);
        let query = SearchQuery {
            pattern: "test".into(),
            ..SearchQuery::default()
        };
        let features = LinearRelevanceModel::extract_features(&entry, &query);
        assert_eq!(features.name_length, 8.0);
        assert!(features.size_log > 0.0);
        assert_eq!(features.name_match, 1.0);
    }

    #[test]
    fn test_score_default_weights() {
        let model = LinearRelevanceModel::new();
        let entry = make_entry("test.txt", 100);
        let query = SearchQuery {
            pattern: "test".into(),
            ..SearchQuery::default()
        };
        let score = model.score_entry(&entry, &query);
        assert!(score >= 0.0);
    }

    #[test]
    fn test_normalize_scores() {
        let mut scores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        LinearRelevanceModel::normalize_scores(&mut scores);
        assert_eq!(scores[0], 0.0);
        assert_eq!(scores[4], 100.0);
    }

    #[test]
    fn test_normalize_empty() {
        let mut scores: Vec<f64> = vec![];
        LinearRelevanceModel::normalize_scores(&mut scores);
        assert!(scores.is_empty());
    }

    #[test]
    fn test_weights_and_bias() {
        let model = LinearRelevanceModel::new();
        assert_eq!(model.weights().len(), 8);
        assert_eq!(model.bias(), -0.2);
    }
}
