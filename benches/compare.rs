use box_large_array::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Standard, Rng, SeedableRng};

const N: usize = 1024 * 1024;

pub fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<u8> = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(10)
        .sample_iter(Standard)
        .take(N)
        .collect();
    let mut group = c.benchmark_group("init");
    group.bench_function("naïve", |b| {
        b.iter(|| naïve::<_, N>(data.iter().copied()))
    });
    group.bench_function("maybe-uninit", |b| {
        b.iter(|| maybe_uninit::<_, N>(data.iter().copied()))
    });
    group.bench_function("not-that-naïve", |b| {
        b.iter(|| not_that_naïve::<_, N>(data.iter().copied()))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
