use criterion::{Criterion, criterion_group, criterion_main};
use onebfc::*;

fn fib_bench(c: &mut Criterion) {
    c.bench_function("fib 1_000_000", |b| b.iter(|| onebfc::fib(1000000)))
        .sample_size(10)
        .warm_up_time(std::time::Duration::from_millis(100));
}

criterion_group!(benches, fib_bench);
criterion_main!(benches);
