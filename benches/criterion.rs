use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tiny_keccak::keccakf;

fn bench_keccak(c: &mut Criterion) {
    const WORDS: usize = 25;

    c.bench_function("keccak", |b| {
        b.iter(|| {
            let mut data = [0u64; WORDS];
            keccakf(black_box(&mut data));
        })
    });
}

criterion_group!(benches, bench_keccak);
criterion_main!(benches);
