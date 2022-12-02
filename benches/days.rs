use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("day1", |b| b.iter(|| advent_of_code::day1()));
  c.bench_function("day2", |b| b.iter(|| advent_of_code::day2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
