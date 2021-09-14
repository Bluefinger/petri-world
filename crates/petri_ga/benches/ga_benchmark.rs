use criterion::{criterion_group, criterion_main, Criterion};
use petri_ga::*;
use petri_rand::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    /// For tests that require access to chromosome
    WithChromosome { chromosome: Chromosome },

    /// For tests that don't require access to chromosome
    WithFitness { fitness: f32 },
}

impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness")
            }
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => {
                chromosome.iter().sum()

                // ^ the simplest fitness function ever - we're just
                // summing all the genes together
            }

            Self::WithFitness { fitness } => *fitness,
        }
    }
}

fn individual(genes: &[f32]) -> TestIndividual {
    let chromosome = genes.iter().cloned().collect();

    TestIndividual::create(chromosome)
}

pub fn selection_benchmark(c: &mut Criterion) {
    c.bench_function("petri-ga select", |b| {
        let roulette = RouletteWheelSelection::new();
        let rng = PetriRand::new();
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
            TestIndividual::new(5.0),
            TestIndividual::new(7.0),
            TestIndividual::new(0.0),
        ];
        let total_weights: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();
        let rate_fn = |individual: &TestIndividual| -> f32 { individual.fitness() / total_weights };
        b.iter(|| roulette.select(&rng, &population, rate_fn));
    });
}

pub fn evolve_benchmark(c: &mut Criterion) {
    c.bench_function("petri-ga evolve", |b| {
        let rng = PetriRand::new();
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );
        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 2.0]), // fitness = 5.0
            individual(&[1.0, 2.0, 5.0]), // fitness = 8.0
        ];
        
        b.iter(|| {
            population = ga.evolve(&rng, &population).expect("not to fail").0;
        });
    });
}

criterion_group!(benches, selection_benchmark, evolve_benchmark);
criterion_main!(benches);
