use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2022::solutions::day01::fibonacci;

pub fn day01_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, day01_fibonacci);

criterion_main!(benches);

