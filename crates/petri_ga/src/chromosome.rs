#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl std::ops::Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl std::iter::FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.genes.iter().zip(other.iter()).all(|(a, b)| (a - b).abs() <= f32::EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        }
    }

    #[test]
    fn len() {
        assert_eq!(chromosome().len(), 3);
    }

    #[test]
    fn iter() {
        let chromosome = chromosome();
        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_relative_eq!(genes[0], &3.0);
        assert_relative_eq!(genes[1], &1.0);
        assert_relative_eq!(genes[2], &2.0);
    }

    #[test]
    fn iter_mut() {
        let mut chromosome = chromosome();

        chromosome.iter_mut().for_each(|gene| {
            *gene *= 10.0;
        });

        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_relative_eq!(genes[0], &30.0);
        assert_relative_eq!(genes[1], &10.0);
        assert_relative_eq!(genes[2], &20.0);
    }

    #[test]
    fn index() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        assert_relative_eq!(chromosome[0], 3.0);
        assert_relative_eq!(chromosome[1], 1.0);
        assert_relative_eq!(chromosome[2], 2.0);
    }

    #[test]
    fn from_iterator() {
        let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

        assert_relative_eq!(chromosome[0], 3.0);
        assert_relative_eq!(chromosome[1], 1.0);
        assert_relative_eq!(chromosome[2], 2.0);
    }

    #[test]
    fn into_iterator() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        let genes: Vec<_> = chromosome.into_iter().collect();

        assert_eq!(genes.len(), 3);
        assert_relative_eq!(genes[0], 3.0);
        assert_relative_eq!(genes[1], 1.0);
        assert_relative_eq!(genes[2], 2.0);
    }
}
