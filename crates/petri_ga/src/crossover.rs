use petri_rand::PetriRand;

use crate::chromosome::Chromosome;

pub trait CrossoverMethod<'a> {
    type Iterator;

    fn crossover(
        &'a self,
        rng: &'a PetriRand,
        parent_a: &'a Chromosome,
        parent_b: &'a Chromosome,
    ) -> Self::Iterator;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UniformCrossover {
    fn default() -> Self {
        UniformCrossover::new()
    }
}

impl<'a> CrossoverMethod<'a> for UniformCrossover {
    type Iterator = impl Iterator<Item = f32> + 'a;

    fn crossover(
        &'a self,
        rng: &'a PetriRand,
        parent_a: &'a Chromosome,
        parent_b: &'a Chromosome,
    ) -> Self::Iterator {
        parent_a.iter()
            .zip(parent_b.iter())
            .map(move |(&a, &b)| if rng.bool() { a } else { b })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_crossover() {
        let rng = PetriRand::with_seed(Default::default());

        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child: Chromosome =
            UniformCrossover::new().crossover(&rng, &parent_a, &parent_b).collect();

        // Number of genes different between `child` and `parent_a`
        let diff_a = child.iter()
            .zip(parent_a)
            .filter(|(c, p)| (*c - p).abs() > f32::EPSILON)
            .count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child.iter()
            .zip(parent_b)
            .filter(|(c, p)| (*c - p).abs() > f32::EPSILON)
            .count();

        // RNG means selection won't always be 50/50 exactly, but given enough
        // inputs, statistically it should be around 50%.
        assert_eq!(diff_a, 41);
        assert_eq!(diff_b, 59);
    }
}
