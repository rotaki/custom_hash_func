// benches/hash_bench.rs
#![allow(clippy::uninlined_format_args)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// ---------------------  your custom hash  ---------------------
#[inline(always)]
fn tiny_murmur(mut k: u64) -> u64 {
    const M: u64 = 0xc6a4a7935bd1e995;
    const R: u32 = 47;
    let mut h = 0x8445d61a4e774912u64 ^ (8u64).wrapping_mul(M);

    k = k.wrapping_mul(M);
    k ^= k >> R;
    k = k.wrapping_mul(M);

    h ^= k;
    h = h.wrapping_mul(M);
    h ^= h >> R;
    h = h.wrapping_mul(M);
    h ^= h >> R;
    h
}

/// ---------------------  benchmarking code  ---------------------
fn hash_benchmarks(c: &mut Criterion) {
    // A small but varied corpus of 64-bit keys.
    let inputs: [u64; 7] = [
        0,
        1,
        0x5555_5555_5555_5555,
        0xDEAD_BEEF_DEAD_BEEF,
        123_456_789,
        u32::MAX as u64,
        u64::MAX,
    ];

    let mut group = c.benchmark_group("u64 hashing");

    for &key in &inputs {
        // ── custom hash ─────────────────────────────────────────
        group.bench_with_input(BenchmarkId::new("tiny_murmur", key), &key, |b, &k| {
            b.iter(|| black_box(tiny_murmur(black_box(k))));
        });

        // ── Rust default hash (SipHash-1-3) ─────────────────────
        group.bench_with_input(BenchmarkId::new("DefaultHasher", key), &key, |b, &k| {
            b.iter(|| {
                let mut h = DefaultHasher::new();
                black_box(k).hash(&mut h);
                black_box(h.finish());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, hash_benchmarks);
criterion_main!(benches);
