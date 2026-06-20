use crate::types::{FileEntry, SearchQuery};

/// A trait for custom ranking algorithms.
pub trait RankingAlgorithm: Send + Sync {
    /// Returns the name of the ranking algorithm.
    fn name(&self) -> &str;

    /// Scores a file entry against a search query.
    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64;

    /// Normalizes scores to be between 0 and 100.
    fn normalize(&self, scores: &mut Vec<f64>) {
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
        } else {
            for score in scores.iter_mut() {
                *score = 50.0;
            }
        }
    }
}

/// Default ranking based on filename and path matching.
pub struct DefaultRanking;

impl RankingAlgorithm for DefaultRanking {
    fn name(&self) -> &str {
        "default"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let pattern = &query.pattern;
        let name = &entry.file_name;
        let path_str = entry.path.to_string_lossy();

        let base_score = 100.0;

        if query.case_sensitive {
            if let Some(pos) = name.find(pattern) {
                let exact_bonus = if name == pattern { 50.0 } else { 0.0 };
                let prefix_bonus = if pos == 0 { 25.0 } else { 0.0 };
                return base_score + exact_bonus + prefix_bonus;
            }
            if path_str.find(pattern).is_some() {
                return base_score * 0.7;
            }
        } else {
            let name_lower = name.to_lowercase();
            let pat_lower = pattern.to_lowercase();
            if let Some(pos) = name_lower.find(&pat_lower) {
                let exact_bonus = if name_lower == pat_lower { 50.0 } else { 0.0 };
                let prefix_bonus = if pos == 0 { 25.0 } else { 0.0 };
                return base_score + exact_bonus + prefix_bonus;
            }
            let path_lower = path_str.to_lowercase();
            if path_lower.find(&pat_lower).is_some() {
                return base_score * 0.7;
            }
        }

        0.0
    }
}

/// TF-IDF inspired ranking based on term frequency in file paths.
pub struct TfIdfRanking;

impl RankingAlgorithm for TfIdfRanking {
    fn name(&self) -> &str {
        "tf-idf"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let pattern = &query.pattern;
        let name = entry.file_name.to_lowercase();
        let path = entry.path.to_string_lossy().to_lowercase();
        let pat_lower = pattern.to_lowercase();

        let tf = name.matches(&pat_lower).count() as f64;
        let path_tf = path.matches(&pat_lower).count() as f64;

        let tf_score = if tf > 0.0 { tf.ln() + 1.0 } else { 0.0 };
        let path_score = if path_tf > 0.0 {
            path_tf.ln() + 1.0
        } else {
            0.0
        };

        let idf = 1.0;

        (tf_score + path_score * 0.5) * idf * 100.0
    }
}

/// BM25-inspired ranking algorithm.
pub struct Bm25Ranking {
    /// Term frequency saturation parameter.
    pub k1: f64,
    /// Length normalization parameter.
    pub b: f64,
}

impl Default for Bm25Ranking {
    fn default() -> Self {
        Self { k1: 1.5, b: 0.75 }
    }
}

impl RankingAlgorithm for Bm25Ranking {
    fn name(&self) -> &str {
        "bm25"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let pattern = &query.pattern;
        let name = entry.file_name.to_lowercase();
        let pat_lower = pattern.to_lowercase();

        let doc_len = name.len() as f64;
        let avg_doc_len = 10.0;

        let tf = name.matches(&pat_lower).count() as f64;
        let numerator = tf * (self.k1 + 1.0);
        let denominator = tf + self.k1 * (1.0 - self.b + self.b * doc_len / avg_doc_len);

        let score = if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        };

        score * 100.0
    }
}

/// Recency-based ranking that favors recently modified files.
pub struct RecencyRanking;

impl RankingAlgorithm for RecencyRanking {
    fn name(&self) -> &str {
        "recency"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let base_score = DefaultRanking.score(entry, query);

        if let Some(modified) = entry.modified {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(modified);
            let days = age.num_days() as f64;

            let recency_factor = if days < 1.0 {
                100.0
            } else if days < 7.0 {
                80.0
            } else if days < 30.0 {
                60.0
            } else if days < 365.0 {
                40.0
            } else {
                20.0
            };

            base_score * 0.5 + recency_factor * 0.5
        } else {
            base_score
        }
    }
}

/// Size-based ranking that favors files within a target size range.
pub struct SizeRanking {
    /// Target minimum size in bytes.
    pub min_size: u64,
    /// Target maximum size in bytes.
    pub max_size: u64,
}

impl Default for SizeRanking {
    fn default() -> Self {
        Self {
            min_size: 1024,
            max_size: 1024 * 1024,
        }
    }
}

impl RankingAlgorithm for SizeRanking {
    fn name(&self) -> &str {
        "size"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        let base_score = DefaultRanking.score(entry, query);

        let size_factor = if entry.size >= self.min_size && entry.size <= self.max_size {
            100.0
        } else if entry.size < self.min_size {
            (entry.size as f64 / self.min_size as f64) * 100.0
        } else {
            (self.max_size as f64 / entry.size as f64) * 100.0
        };

        base_score * 0.5 + size_factor * 0.5
    }
}

/// A combined ranking algorithm that uses multiple algorithms with weights.
pub struct CombinedRanking {
    /// The algorithms and their weights.
    pub algorithms: Vec<(Box<dyn RankingAlgorithm>, f64)>,
}

impl CombinedRanking {
    /// Creates a new empty combined ranking.
    pub fn new() -> Self {
        Self {
            algorithms: Vec::new(),
        }
    }

    /// Adds a ranking algorithm with a weight.
    pub fn add_algorithm(mut self, algorithm: Box<dyn RankingAlgorithm>, weight: f64) -> Self {
        self.algorithms.push((algorithm, weight));
        self
    }
}

impl Default for CombinedRanking {
    fn default() -> Self {
        Self::new()
            .add_algorithm(Box::new(DefaultRanking), 1.0)
            .add_algorithm(Box::new(RecencyRanking), 0.3)
    }
}

impl RankingAlgorithm for CombinedRanking {
    fn name(&self) -> &str {
        "combined"
    }

    fn score(&self, entry: &FileEntry, query: &SearchQuery) -> f64 {
        if self.algorithms.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = self.algorithms.iter().map(|(_, w)| w).sum();
        let weighted_sum: f64 = self
            .algorithms
            .iter()
            .map(|(algo, weight)| algo.score(entry, query) * weight)
            .sum();

        weighted_sum / total_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_entry(name: &str, path: &str, size: u64) -> FileEntry {
        FileEntry {
            id: None,
            path: PathBuf::from(path),
            file_name: name.into(),
            extension: name.rsplit('.').next().map(|s| s.into()),
            parent_dir: PathBuf::from(path).parent().unwrap().to_path_buf(),
            size,
            modified: Some(chrono::Utc::now()),
            accessed: None,
            is_hidden: false,
            is_dir: false,
            hash: None,
        }
    }

    fn make_query(pattern: &str) -> SearchQuery {
        SearchQuery {
            pattern: pattern.into(),
            ..SearchQuery::default()
        }
    }

    #[test]
    fn test_default_ranking_exact_match() {
        let ranking = DefaultRanking;
        let entry = make_entry("main.rs", "/src/main.rs", 100);
        let query = make_query("main.rs");
        let score = ranking.score(&entry, &query);
        assert!(score > 100.0);
    }

    #[test]
    fn test_default_ranking_prefix_match() {
        let ranking = DefaultRanking;
        let entry = make_entry("main.rs", "/src/main.rs", 100);
        let query = make_query("main");
        let score = ranking.score(&entry, &query);
        assert!(score >= 125.0);
    }

    #[test]
    fn test_default_ranking_no_match() {
        let ranking = DefaultRanking;
        let entry = make_entry("test.txt", "/src/test.txt", 100);
        let query = make_query("xyz");
        let score = ranking.score(&entry, &query);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_tf_idf_ranking() {
        let ranking = TfIdfRanking;
        let entry = make_entry("test.rs", "/src/test.rs", 100);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_bm25_ranking() {
        let ranking = Bm25Ranking::default();
        let entry = make_entry("test.rs", "/src/test.rs", 100);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_recency_ranking() {
        let ranking = RecencyRanking;
        let entry = make_entry("test.rs", "/src/test.rs", 100);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_size_ranking() {
        let ranking = SizeRanking::default();
        let entry = make_entry("test.rs", "/src/test.rs", 5000);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_combined_ranking() {
        let ranking = CombinedRanking::default();
        let entry = make_entry("test.rs", "/src/test.rs", 100);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_normalize_scores() {
        let ranking = DefaultRanking;
        let mut scores = vec![100.0, 200.0, 300.0];
        ranking.normalize(&mut scores);
        assert_eq!(scores[0], 0.0);
        assert_eq!(scores[2], 100.0);
    }

    #[test]
    fn test_normalize_empty() {
        let ranking = DefaultRanking;
        let mut scores = vec![];
        ranking.normalize(&mut scores);
        assert!(scores.is_empty());
    }

    #[test]
    fn test_combined_ranking_empty() {
        let ranking = CombinedRanking::new();
        let entry = make_entry("test.rs", "/src/test.rs", 100);
        let query = make_query("test");
        let score = ranking.score(&entry, &query);
        assert_eq!(score, 0.0);
    }
}
