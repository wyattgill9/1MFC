use criterion::{Criterion, criterion_group, criterion_main};
use onebfc::*;

fn fib_bench(c: &mut Criterion) {
    c.bench_function("F(1_000_000)", |b| b.iter(|| code::fib(1000000)));
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(1);
}


criterion_group!(benches, fib_bench);
criterion_main!(benches);
