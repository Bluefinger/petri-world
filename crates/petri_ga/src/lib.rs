#![feature(type_alias_impl_trait)]

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

use petri_rand::PetriRand;
use std::iter::repeat_with;

pub use crate::{chromosome::*, crossover::*, individual::*, mutation::*, selection::*};

pub struct GeneticAlgorithm<S: SelectionMethod, C: CrossoverMethod, M: MutationMethod> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, rng: &PetriRand, population: &[I]) -> Option<Vec<I>>
    where
        I: Individual,
    {
        if population.is_empty() {
            return None;
        }

        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        if total_fitness == 0.0 {
            return None;
        }

        let selection_chance = |individual: &I| -> f32 { individual.fitness() / total_fitness };

        Some(
            repeat_with(|| {
                let parent_a = self
                    .selection_method
                    .select(rng, population, selection_chance)
                    .unwrap()
                    .chromosome();
                let parent_b = self
                    .selection_method
                    .select(rng, population, selection_chance)
                    .unwrap()
                    .chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .take(population.len())
            .collect(),
        )
    }
}

#[cfg(test)]
mod genetic_algorithm {
    use crate::{
        crossover::UniformCrossover, individual::TestIndividual, mutation::GaussianMutation,
        selection::RouletteWheelSelection,
    };

    use super::*;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn evolve_no_fitness() {
        let rng = PetriRand::with_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[0.0, 0.0, 0.0]),
            individual(&[0.0, 0.0, 0.0]),
            individual(&[0.0, 0.0, 0.0]),
        ];

        let result = ga.evolve(&rng, &population);

        assert!(result.is_none());
    }

    #[test]
    fn evolve_no_population() {
        let rng = PetriRand::with_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let population: Vec<TestIndividual> = Vec::new();

        let result = ga.evolve(&rng, &population);

        assert!(result.is_none());
    }

    #[test]
    fn evolve_success() {
        let rng = PetriRand::with_seed(Default::default());

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
        ];

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particular reason for a number of 10 - this test would
        // be fine for 5, 20 or even 1000 generations; the only thing
        // that'd change is the *magnitude* of difference between
        // initial and output population.
        for _ in 0..10 {
            population = ga
                .evolve(&rng, &population)
                .expect("evolution should conclude successfully");
        }

        let expected_population = vec![
            individual(&[1.0694458, 2.1145113, 2.5693974]),
            individual(&[1.3202493, 1.4779799, 2.8062222]),
            individual(&[0.77012694, 1.9899592, 4.6764894]),
            individual(&[1.4511783, 0.075871825, 2.8062222]),
        ];

        assert_eq!(population, expected_population);
    }
}
