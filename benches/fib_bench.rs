use criterion::{Criterion, criterion_group, criterion_main};
use onebfc::*;

// UPDATE THIS WITH YOUR OWN fib()
fn fib_bench(c: &mut Criterion) {
    c.bench_function("fib 1_000_000", |b| b.iter(|| wyattgill9::fib(1000000)));
}

// BASELINE: 
fn fib_bench(c: &mut Criterion) {
    c.bench_function("fib 1_000_000", |b| b.iter(|| code::fib(1000000)));
}

criterion_group!(benches, fib_bench);
criterion_main!(benches);
