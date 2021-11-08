use libplayground::mongodb::{create_test_collection, find_all_docs_in_collection, TEST_COLL_PARAMS};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn find_bench(c: &mut Criterion) {
    // begin setup

    // create the tokio runtime to be used for the benchmarks
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // seed the data server side, get a handle to the collection
    let collection = rt
        .block_on(create_test_collection(TEST_COLL_PARAMS))
        .unwrap();
    // end setup

    c.bench_function("find", |b| {
        b.to_async(&rt).iter(|| {
            // begin measured portion of benchmark
            find_all_docs_in_collection(&collection)
        })
    });
}

criterion_group!(benches, find_bench);
criterion_main!(benches);
