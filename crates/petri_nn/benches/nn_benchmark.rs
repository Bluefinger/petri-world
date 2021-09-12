use criterion::{black_box, criterion_group, criterion_main, Criterion};
use petri_nn::*;
use petri_rand::*;
use std::iter::repeat_with;

pub fn init_benchmark(c: &mut Criterion) {
    c.bench_function("petri-nn random 10/30/10/5", |b| {
        let rng = PetriRand::new();
        let topology = vec![10, 30, 10, 5];
        b.iter(|| Network::random(&rng, black_box(topology.clone())));
    });
}

pub fn propagate_benchmark(c: &mut Criterion) {
    c.bench_function("petri-nn propagate 3/2/1", |b| {
        let rng = PetriRand::new();
        let inputs: Vec<_> = repeat_with(|| rng.get_f32_normalised()).take(3).collect();
        let nn = Network::random(&rng, vec![3, 2, 1]);
        b.iter(|| nn.propagate(black_box(inputs.clone())));
    });
    c.bench_function("petri-nn propagate 15/30/5", |b| {
        let rng = PetriRand::new();
        let inputs: Vec<_> = repeat_with(|| rng.get_f32_normalised()).take(15).collect();
        let nn = Network::random(&rng, vec![15, 30, 5]);
        b.iter(|| nn.propagate(black_box(inputs.to_vec())));
    });
    c.bench_function("petri-nn propagate 10/20/10/5/2", |b| {
        let rng = PetriRand::new();
        let inputs: Vec<_> = repeat_with(|| rng.get_f32_normalised()).take(10).collect();
        let nn = Network::random(&rng, vec![10, 20, 10, 5, 2]);
        b.iter(|| nn.propagate(black_box(inputs.to_vec())));
    });
}

pub fn weights_benchmark(c: &mut Criterion) {
    c.bench_function("petri-nn weights 10/2/1", |b| {
        let rng = PetriRand::new();
        let nn = Network::random(&rng, vec![10, 2, 1]);
        b.iter(|| black_box(nn.weights()));
    });
    c.bench_function("petri-nn from_weights 4/2/2/1", |b| {
        let topology = vec![4, 2, 2, 1];
        let weights = vec![
            0.5, 0.1, -0.3, 0.8, 0.1, 0.2, 0.5, 0.1, -0.3, -0.8, -0.1, 0.2, 0.5, 0.1, 0.4, -0.2,
            0.5, 0.1, 0.4, -0.2, 0.5, 0.1, 0.4, -0.2,
        ];
        b.iter(|| Network::from_weights(black_box(topology.clone()), black_box(weights.clone())));
    });
}

criterion_group!(benches, propagate_benchmark, weights_benchmark, init_benchmark);
criterion_main!(benches);
