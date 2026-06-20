//! Benchmarks for ranking, ML, and semantic features.

use criterion::{Criterion, criterion_group, criterion_main};
use seekr::ml::LinearRelevanceModel;
use seekr::ranking::{Bm25Ranking, CombinedRanking, RankingAlgorithm, TfIdfRanking};
use seekr::semantic::SemanticEncoder;
use seekr::types::{FileEntry, SearchQuery, SearchResult};

fn make_entries(n: usize) -> Vec<FileEntry> {
    (0..n)
        .map(|i| FileEntry {
            id: None,
            path: format!("/tmp/file_{}.txt", i).into(),
            file_name: format!("file_{}.txt", i),
            size: (i * 100) as u64,
            extension: Some("txt".into()),
            is_dir: false,
            is_hidden: false,
            modified: None,
            parent_dir: "/tmp".into(),
            accessed: None,
            hash: None,
        })
        .collect()
}

fn make_results(entries: &[FileEntry]) -> Vec<SearchResult> {
    entries
        .iter()
        .map(|e| SearchResult {
            entry: e.clone(),
            score: 0.0,
            matched_indices: vec![],
        })
        .collect()
}

fn bench_ranking(c: &mut Criterion) {
    let entries = make_entries(1000);
    let results = make_results(&entries);
    let query = SearchQuery {
        pattern: "file_500".into(),
        ..SearchQuery::default()
    };

    c.bench_function("tfidf_ranking_1000", |b| {
        let ranker = TfIdfRanking;
        b.iter(|| {
            let mut r = results.clone();
            for item in &mut r {
                item.score = ranker.score(&item.entry, &query);
            }
            r
        });
    });

    c.bench_function("bm25_ranking_1000", |b| {
        let ranker = Bm25Ranking::default();
        b.iter(|| {
            let mut r = results.clone();
            for item in &mut r {
                item.score = ranker.score(&item.entry, &query);
            }
            r
        });
    });

    c.bench_function("combined_ranking_1000", |b| {
        let ranker = CombinedRanking::default();
        b.iter(|| {
            let mut r = results.clone();
            for item in &mut r {
                item.score = ranker.score(&item.entry, &query);
            }
            r
        });
    });
}

fn bench_ml(c: &mut Criterion) {
    let entries = make_entries(100);
    let model = LinearRelevanceModel::new();
    let query = SearchQuery {
        pattern: "file_50".into(),
        ..SearchQuery::default()
    };

    c.bench_function("ml_score_100_entries", |b| {
        b.iter(|| {
            for entry in &entries {
                model.score_entry(entry, &query);
            }
        });
    });
}

fn bench_semantic(c: &mut Criterion) {
    let entries = make_entries(500);
    let encoder = SemanticEncoder::build(&entries);
    let query = SearchQuery {
        pattern: "file_250".into(),
        ..SearchQuery::default()
    };

    c.bench_function("semantic_similarity_500", |b| {
        b.iter(|| {
            for entry in &entries {
                encoder.similarity(&query, entry);
            }
        });
    });
}

criterion_group!(benches, bench_ranking, bench_ml, bench_semantic);
criterion_main!(benches);
