use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use seekr::core::SeekrApp;
use seekr::types::{AppConfig, SearchQuery};
use std::fs;
use std::path::PathBuf;

fn setup_index() -> (SeekrApp, PathBuf) {
    let dir = std::env::temp_dir().join("seekr_bench");
    let _ = fs::create_dir_all(&dir);

    for i in 0..1000 {
        let _ = fs::write(
            dir.join(format!("file_{}.txt", i)),
            format!("content {}", i),
        );
    }

    let sub = dir.join("subdir");
    let _ = fs::create_dir_all(&sub);
    for i in 0..500 {
        let _ = fs::write(
            sub.join(format!("nested_{}.rs", i)),
            format!("fn func_{}() {{}}", i),
        );
    }

    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })
    .unwrap();

    app.index(&dir).unwrap();
    (app, dir)
}

fn bench_search(c: &mut Criterion) {
    let (app, dir) = setup_index();

    c.bench_function("search_filename", |b| {
        b.iter(|| {
            let query = SearchQuery {
                pattern: "file_500".into(),
                ..SearchQuery::default()
            };
            app.search(black_box(&query)).unwrap()
        })
    });

    c.bench_function("search_regex", |b| {
        b.iter(|| {
            let query = SearchQuery {
                pattern: r"file_\d+\.txt".into(),
                use_regex: true,
                limit: Some(100),
                ..SearchQuery::default()
            };
            app.search(black_box(&query)).unwrap()
        })
    });

    c.bench_function("search_fuzzy", |b| {
        b.iter(|| app.fuzzy_search(black_box("file"), 100).unwrap())
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_index(c: &mut Criterion) {
    c.bench_function("index_directory", |b| {
        b.iter(|| {
            let dir = std::env::temp_dir().join("seekr_bench_idx");
            let _ = fs::create_dir_all(&dir);
            for i in 0..200 {
                let _ = fs::write(dir.join(format!("f{}.txt", i)), "data");
            }

            let app = SeekrApp::new(AppConfig {
                database_path: None,
                ..AppConfig::default()
            })
            .unwrap();

            app.index(black_box(&dir)).unwrap();
            let _ = fs::remove_dir_all(&dir);
        })
    });
}

criterion_group!(benches, bench_search, bench_index);
criterion_main!(benches);
