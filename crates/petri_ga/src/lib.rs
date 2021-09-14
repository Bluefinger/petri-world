#![feature(type_alias_impl_trait)]

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;

use petri_rand::PetriRand;
use std::{iter::repeat_with, marker::PhantomData};

pub use crate::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*, statistics::*,
};

#[derive(Debug)]
pub struct GeneticAlgorithm<'a, S: SelectionMethod, C: CrossoverMethod<'a>, M: MutationMethod> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
    _marker: PhantomData<&'a C>,
}

impl<'a, S, C, M> GeneticAlgorithm<'a, S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod<'a>,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
            _marker: PhantomData,
        }
    }

    pub fn evolve<I>(
        &'a self,
        rng: &'a PetriRand,
        population: &'a [I],
    ) -> Option<(Vec<I>, Statistics)>
    where
        I: Individual,
        <C as crossover::CrossoverMethod<'a>>::Iterator: std::iter::Iterator<Item = f32>,
    {
        if population.is_empty() {
            return None;
        }

        let stats = StatisticsBuilder::from_population(population);

        if stats.has_no_fitness() {
            return None;
        }

        let selection_chance =
            |individual: &I| -> f32 { individual.fitness() / stats.total_fitness() };

        let new_population = repeat_with(|| {
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

            let child = self.mutation_method.mutate(
                rng,
                self.crossover_method.crossover(rng, parent_a, parent_b),
            );

            I::create(child)
        })
        .take(population.len())
        .collect();

        Some((new_population, stats))
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
    fn crossover_and_mutate() {
        let rng = PetriRand::with_seed(Default::default());
        let crosser = UniformCrossover::new();
        let mutator = GaussianMutation::new(0.5, 0.5);

        let parent_a = individual(&[1.0, 2.0, 3.0, 4.0, 5.0]).chromosome().clone();
        let parent_b = individual(&[-1.0, -2.0, -3.0, -4.0, -5.0])
            .chromosome()
            .clone();

        let child = mutator.mutate(&rng, crosser.crossover(&rng, &parent_a, &parent_b));

        let expected_child = individual(&[1.0, 1.9195144, -2.9491906, 4.0, 5.0])
            .chromosome()
            .clone();

        assert_eq!(child, expected_child);
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
                .expect("evolution should conclude successfully")
                .0;
        }

        let expected_population = vec![
            individual(&[1.9979382, 1.6373904, 2.3867228]),
            individual(&[2.3059247, 1.3087935, 2.5775788]),
            individual(&[1.8600383, 1.6250226, 2.5775788]),
            individual(&[1.8600383, 1.2199795, 2.2783923]),
        ];

        assert_eq!(population, expected_population);
    }
}
