use petri_rand::PetriRand;
use std::iter::repeat_with;

use crate::individual::Individual;

pub trait SelectionMethod {
    fn select<'a, I, F>(&self, rng: &PetriRand, population: &'a [I], selection_chance: F) -> Option<&'a I>
    where
        I: Individual,
        F: Fn(&I) -> f32;
}

#[derive(Debug)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RouletteWheelSelection {
    fn default() -> Self {
        RouletteWheelSelection::new()
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I, F>(&self, rng: &PetriRand, population: &'a [I], selection_chance: F) -> Option<&'a I>
    where
        I: Individual,
        F: Fn(&I) -> f32,
    {
        repeat_with(|| rng.sample(population))
            .flatten()
            .find(|&individual| rng.chance((selection_chance(individual)) as _))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::individual::TestIndividual;
    use std::{collections::BTreeMap, iter::FromIterator};

    #[test]
    fn roulette_wheel_selection() {
        let method = RouletteWheelSelection::new();
        let rng = PetriRand::with_seed(Default::default());

        let population = vec![
            TestIndividual::new(0.0),
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram: BTreeMap<i32, _> = repeat_with(|| {
            method.select(&rng, &population, |individual| individual.fitness() / 10.0).unwrap()
        })
        .take(1000)
        .fold(BTreeMap::from_iter(vec![(0, 0)]), |mut histogram, individual| {
            *histogram.entry(individual.fitness() as _).or_default() += 1;

            histogram
        });

        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (0, 0),
            (1, 99),
            (2, 173),
            (3, 300),
            (4, 428),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
