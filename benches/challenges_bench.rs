use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_challenges::week1::bahoang::Solution_BaHoang;
use rust_challenges::week1::cuong::Solution_Cuong;
use rust_challenges::week1::long::Solution_Long;
use rust_challenges::week1::nhatcuong_4920::Solution_C;
use rust_challenges::week1::nhatquan::Solution_NhatQuan;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("week1");

  let low = 10;
  let high = 10000;

  for i in [1, 2, 3, 4, 5].iter() {
    group.bench_with_input(BenchmarkId::new("nhatquan", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution_NhatQuan::strobogrammatic_numbers(*low, *high))
    });

    // group.bench_with_input(BenchmarkId::new("nhatcuong", i), &(low, high), |b, (low, high)| {
    //   b.iter(|| Solution_C::strobogrammatic_numbers(*low, *high))
    // });

    group.bench_with_input(BenchmarkId::new("bahoang", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution_BaHoang::strobogrammatic_numbers(*low, *high))
    });

    group.bench_with_input(BenchmarkId::new("cuong", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution_Cuong::strobogrammatic_numbers(*low, *high))
    });

    group.bench_with_input(BenchmarkId::new("Long", i), &(low, high), |b, (low, high)| {
      b.iter(|| Solution_Long::strobogrammatic_numbers(*low, *high))
    });
  }
  group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
