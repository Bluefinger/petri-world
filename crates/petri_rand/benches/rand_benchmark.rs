use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastrand::{Rng as FastRng};
use wyhash::WyRng;
use petri_rand::*;
use rand::prelude::*;

pub fn gen_u32_speed(c: &mut Criterion) {
    c.bench_function("petri-rand u32", |b| {
        let rng = PetriRand::new();

        b.iter(|| {
            let mut sum = black_box(0u32);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.get_u32())
            }
            sum
        });
    });

    c.bench_function("fastrand u32", |b| {
        let rng = FastRng::new();

        b.iter(|| {
            let mut sum = black_box(0u32);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.u32(..))
            }
            sum
        });
    });

    c.bench_function("wyhash u32", |b| {
        let mut rng = WyRng::from_rng(thread_rng()).unwrap();
        b.iter(|| {
            let mut sum = 0u32;
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.gen::<u32>());
            }
            sum
        });
    });
}

pub fn gen_u64_speed(c: &mut Criterion) {
    c.bench_function("petri-rand u64", |b| {
        let rng = PetriRand::new();

        b.iter(|| {
            let mut sum = black_box(0u64);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.get_u64())
            }
            sum
        });
    });

    c.bench_function("fastrand u64", |b| {
        let rng = FastRng::new();

        b.iter(|| {
            let mut sum = black_box(0u64);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.u64(..))
            }
            sum
        });
    });

    c.bench_function("wyhash u64", |b| {
        let mut rng = WyRng::from_rng(thread_rng()).unwrap();
        b.iter(|| {
            let mut sum = 0u64;
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.gen::<u64>());
            }
            sum
        });
    });
}

pub fn gen_usize_speed(c: &mut Criterion) {
    c.bench_function("petri-rand usize", |b| {
        let rng = PetriRand::new();

        b.iter(|| {
            let mut sum = black_box(0usize);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.index(..100))
            }
            sum
        });
    });

    c.bench_function("fastrand usize", |b| {
        let rng = FastRng::new();

        b.iter(|| {
            let mut sum = black_box(0usize);
            for _ in 0..10_000 {
                sum = sum.wrapping_add(rng.usize(..100))
            }
            sum
        });
    });
}

pub fn gen_f32_speed(c: &mut Criterion) {
    c.bench_function("petri-rand f32", |b| {
        let rng = PetriRand::new();

        b.iter(|| {
            let mut sum = black_box(0.0f32);
            for _ in 0..10_000 {
                sum += rng.get_f32();
            }
            sum
        });
    });

    c.bench_function("fastrand f32", |b| {
        let rng = FastRng::new();

        b.iter(|| {
            let mut sum = black_box(0.0f32);
            for _ in 0..10_000 {
                sum += rng.f32();
            }
            sum
        });
    });
}

criterion_group!(
    benches,
    gen_u32_speed,
    gen_u64_speed,
    gen_usize_speed,
    gen_f32_speed
);
criterion_main!(benches);
