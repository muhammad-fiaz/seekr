//! Demonstrates ranking strategies.

use seekr::ranking::{Bm25Ranking, CombinedRanking, RankingAlgorithm, TfIdfRanking};
use seekr::types::{FileEntry, SearchQuery, SearchResult};

fn make_result(name: &str, size: u64) -> SearchResult {
    SearchResult {
        entry: FileEntry {
            id: None,
            path: format!("/tmp/{}", name).into(),
            file_name: name.into(),
            size,
            extension: name.split('.').next_back().map(|s| s.to_string()),
            is_dir: false,
            is_hidden: false,
            modified: None,
            parent_dir: "/tmp".into(),
            accessed: None,
            hash: None,
        },
        score: 0.0,
        matched_indices: vec![],
    }
}

fn main() {
    let results = vec![
        make_result("test_file.rs", 500),
        make_result("another_test.rs", 1000),
        make_result("testing.rs", 200),
    ];

    let tfidf = TfIdfRanking;
    let bm25 = Bm25Ranking::default();
    let combined = CombinedRanking::default();

    let query = SearchQuery {
        pattern: "test".into(),
        ..SearchQuery::default()
    };

    println!("TF-IDF ranking:");
    let mut scored = results.clone();
    for r in &mut scored {
        r.score = tfidf.score(&r.entry, &query);
    }
    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for r in &scored {
        println!("  {:.2} {}", r.score, r.entry.file_name);
    }

    println!("\nBM25 ranking:");
    let mut scored = results.clone();
    for r in &mut scored {
        r.score = bm25.score(&r.entry, &query);
    }
    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for r in &scored {
        println!("  {:.2} {}", r.score, r.entry.file_name);
    }

    println!("\nCombined ranking:");
    let mut scored = results.clone();
    for r in &mut scored {
        r.score = combined.score(&r.entry, &query);
    }
    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for r in &scored {
        println!("  {:.2} {}", r.score, r.entry.file_name);
    }
}
