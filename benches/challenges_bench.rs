use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_challenges::week1::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("week1");

  let low = 10;
  let high = 10000;

  for i in [20u64, 21u64].iter() {
    group.bench_with_input(BenchmarkId::new("person 1", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution::strobogrammatic_numbers(*low, *high))
    });

    group.bench_with_input(BenchmarkId::new("person 2", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution::strobogrammatic_numbers(*low, *high))
    });
  }
  group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
