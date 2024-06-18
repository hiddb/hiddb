use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hiddb::db::dbtypes::*;
use hiddb::db::RocksDB;
use hiddb::distance::*;
use hiddb::hiddb::{collection, document, index};
use hiddb::hnsw::Document;
use hiddb::hnsw::IndexBuilder;
use hiddb::index_store;
use serde_json::*;

use rand::prelude::*;

use hiddb::hnsw::document::*;
use rand::distributions::Uniform;

// TODO: do benchmark properly
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distance Metrics");

    let mut rng = rand::thread_rng();
    let vector_a: Vec<f64> = (0..300).map(|_| rng.gen()).collect();
    let vector_b: Vec<f64> = (0..300).map(|_| rng.gen()).collect();
    group.bench_function("Squared eucledian distance", |b| {
        b.iter(|| squared_euclidean(black_box(&vector_a), black_box(&vector_b)))
    });

    let vector_a: Vec<f64> = (0..300).map(|_| rng.gen()).collect();
    let vector_b: Vec<f64> = (0..300).map(|_| rng.gen()).collect();
    group.bench_function("Eucledian distance", |b| b.iter(|| euclidean(black_box(&vector_a), black_box(&vector_b))));

    let db_options;
    {
        let db = &RocksDB::init("./build/benchmark.rdb");
        db_options = db.options.clone();

        let index_store = index_store::init(db);

        let collection_name = "test_collection".to_owned();
        let field_name = "vector";

        let test_collection = Collection {
            collection_id: collection_name.clone(),
            n_documents: 0,
        };

        assert_eq!(
            collection::create(db, &collection_name).unwrap().collection_id,
            test_collection.collection_id
        );

        index::create(db, &index_store, &collection_name, field_name, 200).unwrap();

        let mut rng = rand::thread_rng();
        for _ in 0..500 {}
        group.sample_size(10);

        group.bench_function("Insert", |b| {
            b.iter(|| {
                for _ in 0..500 {
                    let range = Uniform::new(-100.0, 100.0);
                    let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();
                    let document = json!({"id": rng.gen::<u64>().to_string(), "vector": vector});

                    document::insert(
                        black_box(db),
                        black_box(&index_store),
                        black_box(&test_collection.collection_id),
                        black_box(&vec![document.clone()]),
                    )
                    .unwrap();
                }
            })
        });
        let range = Uniform::new(-100.0, 100.0);
        let vector: Vec<f64> = (0..200).map(|_| rng.sample(&range)).collect();
        group.bench_function("Search", |b| {
            b.iter(|| {
                for _ in 0..500 {
                    // hnsw_index.knn_search(black_box(&db), black_box(&vector), black_box(20));
                    document::search_ann(
                        &db,
                        &index_store,
                        &collection_name,
                        &json!({"field_name": "vector", "vectors": vec![vector.clone()]}),
                    )
                    .unwrap();
                }
            })
        });
    }
    RocksDB::destroy(&db_options, "benchmark.rdb");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
