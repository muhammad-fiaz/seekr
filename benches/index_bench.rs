use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use seekr::core::SeekrApp;
use seekr::indexer;
use seekr::types::{AppConfig, IndexerConfig};
use std::fs;
use std::path::PathBuf;

fn setup_dir() -> PathBuf {
    let dir = std::env::temp_dir().join("seekr_index_bench");
    let _ = fs::create_dir_all(&dir);

    for i in 0..5000 {
        let _ = fs::write(dir.join(format!("file_{}.txt", i)), format!("data {}", i));
    }

    for i in 0..10 {
        let sub = dir.join(format!("dir_{}", i));
        let _ = fs::create_dir_all(&sub);
        for j in 0..100 {
            let _ = fs::write(sub.join(format!("f{}.rs", j)), "fn foo() {}");
        }
    }

    dir
}

fn bench_full_index(c: &mut Criterion) {
    let dir = setup_dir();

    c.bench_function("full_index_5000_files", |b| {
        b.iter(|| {
            let app = SeekrApp::new(AppConfig {
                database_path: None,
                ..AppConfig::default()
            })
            .unwrap();

            let config = IndexerConfig::default();
            indexer::index_directory(black_box(&app.database()), &dir, &config).unwrap()
        })
    });

    let _ = fs::remove_dir_all(&dir);
}

fn bench_incremental_index(c: &mut Criterion) {
    let dir = setup_dir();
    let app = SeekrApp::new(AppConfig {
        database_path: None,
        ..AppConfig::default()
    })
    .unwrap();
    app.index(&dir).unwrap();

    c.bench_function("incremental_index", |b| {
        b.iter(|| {
            let since = chrono::Utc::now() - chrono::Duration::hours(1);
            app.index_incremental(black_box(&dir), since).unwrap()
        })
    });

    let _ = fs::remove_dir_all(&dir);
}

criterion_group!(benches, bench_full_index, bench_incremental_index);
criterion_main!(benches);
