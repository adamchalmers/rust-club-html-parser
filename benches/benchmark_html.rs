use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, BuildHasherDefault},
};

use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};
use html_parser::Tag;
use rustc_hash::FxHasher;
use winnow::Parser;

fn generate_sample_input(num_attributes: usize) -> String {
    let kvs: Vec<_> = (0..num_attributes).map(|_| r#"width="40""#).collect();
    let attributes = kvs.join(", ");
    format!("<div {attributes}>")
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse HTML div");
    for size in [2, 4, 16, 32, 64, 128, 256].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("RandomState", size),
            size,
            run_bench::<RandomState>,
        );
        group.bench_with_input(
            BenchmarkId::new("FxHashMap", size),
            size,
            run_bench::<BuildHasherDefault<FxHasher>>,
        );
    }
    group.finish();
}

fn run_bench<S>(b: &mut Bencher, size: &usize)
where
    S: BuildHasher + Default,
{
    // Set up benchmark
    // We don't want to benchmark generating the input!
    let input = generate_sample_input(*size);
    // Run benchmark and gather data
    b.iter(|| black_box(Tag::<S>::parse.parse(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
