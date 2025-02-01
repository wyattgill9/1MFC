use criterion::{Criterion, criterion_group, criterion_main};
use onebfc::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("F(1_000_000)", |b| b.iter(|| code::fib(1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
