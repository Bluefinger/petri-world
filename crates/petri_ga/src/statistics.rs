use crate::Individual;

#[derive(Debug, Clone)]
pub struct StatisticsBuilder {
    min_fitness: f32,
    max_fitness: f32,
    sum_fitness: f32,
    total_samples: usize,
}

impl StatisticsBuilder {
    pub fn new() -> Self {
        Self {
            min_fitness: f32::MAX,
            max_fitness: 0.0,
            sum_fitness: 0.0,
            total_samples: 0,
        }
    }

    pub fn add<I>(&mut self, individual: &I)
    where
        I: Individual,
    {
        let fitness = individual.fitness();

        self.min_fitness = self.min_fitness.min(fitness);
        self.max_fitness = self.max_fitness.max(fitness);
        self.sum_fitness += fitness;
        self.total_samples += 1;
    }

    pub fn build(self) -> Statistics {
        Statistics {
            min_fitness: self.min_fitness,
            max_fitness: self.max_fitness,
            total_fitness: self.sum_fitness,
            avg_fitness: self.sum_fitness / (self.total_samples as f32),
        }
    }
}

impl Default for StatisticsBuilder {
    fn default() -> Self {
        StatisticsBuilder::new()
    }
}

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    total_fitness: f32,
    avg_fitness: f32,
}

impl Statistics {
    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }

    pub fn total_fitness(&self) -> f32 {
        self.total_fitness
    }
}
