use std::collections::HashMap;

use crate::types::{FileEntry, SearchQuery};

/// A simple TF-IDF based semantic encoder.
pub struct SemanticEncoder {
    /// Document frequency map.
    document_freq: HashMap<String, u64>,
    /// Total number of documents.
    total_documents: u64,
}

impl SemanticEncoder {
    /// Creates a new semantic encoder.
    pub fn new() -> Self {
        Self {
            document_freq: HashMap::new(),
            total_documents: 0,
        }
    }

    /// Builds the encoder from a set of file entries.
    pub fn build(entries: &[FileEntry]) -> Self {
        let mut encoder = Self::new();
        let mut term_docs: HashMap<String, u64> = HashMap::new();

        for entry in entries {
            let terms = Self::tokenize_entry(entry);
            let mut seen = std::collections::HashSet::new();
            for term in &terms {
                if seen.insert(term.clone()) {
                    *term_docs.entry(term.clone()).or_insert(0) += 1;
                }
            }
            encoder.total_documents += 1;
        }

        encoder.document_freq = term_docs;
        encoder
    }

    /// Tokenizes a file entry into terms.
    fn tokenize_entry(entry: &FileEntry) -> Vec<String> {
        let mut terms = Vec::new();

        for word in entry.file_name.split(|c: char| !c.is_alphanumeric()) {
            if !word.is_empty() {
                terms.push(word.to_lowercase());
            }
        }

        for word in entry
            .path
            .to_string_lossy()
            .split(|c: char| !c.is_alphanumeric())
        {
            if !word.is_empty() {
                terms.push(word.to_lowercase());
            }
        }

        if let Some(ref ext) = entry.extension {
            terms.push(ext.clone());
        }

        terms
    }

    /// Tokenizes a search query.
    fn tokenize_query(query: &SearchQuery) -> Vec<String> {
        query
            .pattern
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect()
    }

    /// Computes TF-IDF vector for a set of terms.
    fn compute_tfidf(&self, terms: &[String]) -> HashMap<String, f64> {
        let mut tf: HashMap<String, f64> = HashMap::new();
        for term in terms {
            *tf.entry(term.clone()).or_insert(0.0) += 1.0;
        }

        let total = terms.len() as f64;
        let mut tfidf = HashMap::new();

        for (term, count) in &tf {
            let tf_val = count / total;
            let df = self.document_freq.get(term).copied().unwrap_or(0) as f64;
            let idf = if df > 0.0 {
                (self.total_documents as f64 / df).ln() + 1.0
            } else {
                1.0
            };
            tfidf.insert(term.clone(), tf_val * idf);
        }

        tfidf
    }

    /// Computes cosine similarity between two TF-IDF vectors.
    fn cosine_similarity(a: &HashMap<String, f64>, b: &HashMap<String, f64>) -> f64 {
        let mut dot_product = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (term, val) in a {
            norm_a += val * val;
            if let Some(b_val) = b.get(term) {
                dot_product += val * b_val;
            }
        }

        for val in b.values() {
            norm_b += val * val;
        }

        let norm_a = norm_a.sqrt();
        let norm_b = norm_b.sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            dot_product / (norm_a * norm_b)
        } else {
            0.0
        }
    }

    /// Computes semantic similarity between a query and a file entry.
    pub fn similarity(&self, query: &SearchQuery, entry: &FileEntry) -> f64 {
        let query_terms = Self::tokenize_query(query);
        let entry_terms = Self::tokenize_entry(entry);

        let query_tfidf = self.compute_tfidf(&query_terms);
        let entry_tfidf = self.compute_tfidf(&entry_terms);

        Self::cosine_similarity(&query_tfidf, &entry_tfidf) * 100.0
    }

    /// Returns the total number of documents indexed.
    pub fn total_documents(&self) -> u64 {
        self.total_documents
    }

    /// Returns the document frequency for a term.
    pub fn document_frequency(&self, term: &str) -> u64 {
        self.document_freq.get(term).copied().unwrap_or(0)
    }
}

impl Default for SemanticEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(name: &str, path: &str) -> FileEntry {
        FileEntry {
            id: None,
            path: path.into(),
            file_name: name.into(),
            size: 100,
            extension: name.split('.').next_back().map(|s| s.to_string()),
            is_dir: false,
            is_hidden: false,
            modified: None,
            parent_dir: "/tmp".into(),
            accessed: None,
            hash: None,
        }
    }

    #[test]
    fn test_build_encoder() {
        let entries = vec![
            make_entry("hello_world.txt", "/tmp/hello_world.txt"),
            make_entry("foo_bar.rs", "/tmp/foo_bar.rs"),
        ];
        let encoder = SemanticEncoder::build(&entries);
        assert_eq!(encoder.total_documents(), 2);
    }

    #[test]
    fn test_similarity_matching() {
        let entries = vec![
            make_entry("hello_world.txt", "/tmp/hello_world.txt"),
            make_entry("goodbye_world.txt", "/tmp/goodbye_world.txt"),
        ];
        let encoder = SemanticEncoder::build(&entries);
        let query = SearchQuery {
            pattern: "hello".into(),
            ..SearchQuery::default()
        };
        let sim0 = encoder.similarity(&query, &entries[0]);
        let sim1 = encoder.similarity(&query, &entries[1]);
        assert!(sim0 > sim1);
    }

    #[test]
    fn test_document_frequency() {
        let entries = vec![
            make_entry("test.txt", "/tmp/test.txt"),
            make_entry("test.rs", "/tmp/test.rs"),
        ];
        let encoder = SemanticEncoder::build(&entries);
        assert!(encoder.document_frequency("test") > 0);
    }

    #[test]
    fn test_new_encoder() {
        let encoder = SemanticEncoder::new();
        assert_eq!(encoder.total_documents(), 0);
    }
}
