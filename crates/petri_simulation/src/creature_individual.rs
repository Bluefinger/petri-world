use petri_ga::{Chromosome, Individual};
use petri_nn::Network;

use crate::*;

#[derive(Debug, Clone)]
pub struct CreatureIndividual {
    pub fitness: f32,
    pub chromosome: Chromosome,
}

impl CreatureIndividual {
    pub fn from_creature(brain: &Network, fitness: &Fitness) -> Self {
        Self {
            fitness: fitness.score,
            chromosome: brain.weights().collect(),
        }
    }
}

impl Individual for CreatureIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}

impl IntoIterator for CreatureIndividual {
    type Item = f32;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chromosome.into_iter()
    }
}
