#![feature(array_windows)]

use petri_rand::*;
use std::iter::{once, repeat, repeat_with};

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<usize>,
    neurons: Vec<Neuron>,
}

#[derive(Debug, Clone)]
struct Neuron {
    bias: f32,
    coefficient: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(rng: &PetriRand, layers: Vec<usize>) -> Self {
        debug_assert!(layers.len() > 1);

        let neurons = layers
            .array_windows()
            .flat_map(|&[input, output]| repeat(input).take(output))
            .map(|input| Neuron::random(rng, input))
            .collect();

        Self { neurons, layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let (result, _) =
            self.layers
                .array_windows()
                .fold((inputs, 0), |(inputs, from), [_, output]| {
                    let to = from + output;

                    (
                        self.neurons[from..to]
                            .iter()
                            .map(|neuron| neuron.propagate(&inputs))
                            .collect(),
                        to,
                    )
                });

        result
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.neurons
            .iter()
            .flat_map(|neuron| neuron.weights())
            .copied()
    }

    pub fn adjust_weights(&mut self, weights: impl IntoIterator<Item = f32>) {
        let mut weights = weights.into_iter();

        self.neurons = self
            .layers
            .array_windows()
            .flat_map(|&[input, output]| repeat(input).take(output))
            .map(|input| Neuron::from_weights(input, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("Too many weights given");
        }
    }

    pub fn from_weights(layers: Vec<usize>, weights: impl IntoIterator<Item = f32>) -> Self {
        debug_assert!(layers.len() > 1);

        //let layers = topology.array_windows();

        let mut weights = weights.into_iter();

        let neurons = layers
            .array_windows()
            .flat_map(|&[input, output]| repeat(input).take(output))
            .map(|input| Neuron::from_weights(input, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("Too many weights given");
        }

        Self { layers, neurons }
    }
}

impl Neuron {
    pub fn random(rng: &PetriRand, input_size: usize) -> Self {
        let bias = rng.get_f32_normalised();

        let coefficient = rng.get_f32_normalised();

        let weights = repeat_with(|| rng.get_f32_normalised())
            .take(input_size)
            .collect();

        Self {
            bias,
            coefficient,
            weights,
        }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        debug_assert!(inputs.len() == self.weights.len());

        let output = self.bias
            + inputs
                .iter()
                .zip(&self.weights)
                .map(|(input, weight)| input * weight)
                .sum::<f32>();

        if output > 0.0 {
            output
        } else {
            self.coefficient * output
        }
    }

    pub fn weights(&self) -> impl Iterator<Item = &f32> + '_ {
        once(&self.bias)
            .chain(once(&self.coefficient))
            .chain(self.weights.iter())
    }

    pub fn from_weights(input_neurons: usize, weights: &mut impl Iterator<Item = f32>) -> Self {
        let bias = weights.next().unwrap();
        let coefficient = weights.next().unwrap();

        let weights = repeat_with(|| weights.next().unwrap())
            .take(input_neurons)
            .collect();

        Self {
            bias,
            weights,
            coefficient,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::*;

    mod neuron {
        use super::*;

        #[test]
        fn random() {
            // Because we always use the same seed, our `rng` in here will
            // always return the same set of values
            let rng = PetriRand::with_seed(Default::default());
            let neuron = Neuron::random(&rng, 4);

            assert_relative_eq!(neuron.bias, 0.119923115);
            assert_relative_eq!(neuron.coefficient, 0.9945836);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.11937928, 0.13722491, 0.38501358, -0.67805743].as_ref()
            );
        }

        #[test]
        fn propagate() {
            let neuron = Neuron {
                bias: 0.5,
                coefficient: 0.1,
                weights: vec![-0.3, 0.8],
            };

            // Ensures our PReLU works for < 0.0 cases:
            assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]),
                ((-0.3 * -10.0) + (0.8 * -10.0) + 0.5) * 0.1
            );

            // `0.5` and `1.0` chosen by a fair dice roll:
            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }

        #[test]
        fn from_weights() {
            let weights: Vec<f32> = vec![0.5, 0.1, 0.9, 0.8, -0.1];

            let neuron = Neuron::from_weights(3, &mut weights.into_iter());

            assert_relative_eq!(neuron.bias, 0.5);
            assert_relative_eq!(neuron.coefficient, 0.1);
            assert_relative_eq!(neuron.weights.as_slice(), [0.9, 0.8, -0.1].as_ref());
        }
    }

    mod network {
        use super::*;

        #[test]
        fn random() {
            let topology = vec![4, 2];
            let rng = PetriRand::with_seed(Default::default());

            let network = Network::random(&rng, topology);

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.neurons[0].weights.len(), 4);
        }

        #[test]
        fn propagate() {
            let inputs = vec![0.5, 1.0, 0.75];
            let layers = vec![3, 2, 1];

            let network = Network {
                layers,
                neurons: vec![
                    Neuron {
                        bias: 0.5,
                        coefficient: 0.1,
                        weights: vec![-0.3, 0.8, 0.1],
                    },
                    Neuron {
                        bias: 0.5,
                        coefficient: 0.1,
                        weights: vec![-0.3, -0.8, -0.1],
                    },
                    Neuron {
                        bias: 0.5,
                        coefficient: 0.1,
                        weights: vec![0.4, -0.2],
                    },
                ],
            };

            let result = network.propagate(inputs);

            // Manually math out the neural network results
            let input1 = (-0.3 * 0.5) + (0.8 * 1.0) + (0.1 * 0.75) + 0.5; // 1.2249999
            let input2 = ((-0.3 * 0.5) + (-0.8 * 1.0) + (-0.1 * 0.75) + 0.5) * 0.1; // -0.0525 ish
            let input3 = (0.4 * input1) + (-0.2 * input2) + 0.5; // 1.0005

            let expected = vec![input3];

            assert_relative_eq!(result.as_slice(), expected.as_slice());
        }

        #[test]
        fn weights() {
            let layers = vec![2, 1];
            let network = Network {
                layers,
                neurons: vec![Neuron {
                    bias: 0.5,
                    coefficient: 0.1,
                    weights: vec![-0.3, 0.8],
                }],
            };

            let mut network_weights = network.weights();

            assert_relative_eq!(network_weights.next().unwrap(), 0.5);
            assert_relative_eq!(network_weights.next().unwrap(), 0.1);
            assert_relative_eq!(network_weights.next().unwrap(), -0.3);
            assert_relative_eq!(network_weights.next().unwrap(), 0.8);
            assert_eq!(network_weights.next(), None);
        }

        #[test]
        fn from_weights() {
            let topology = vec![4, 1];
            let weights: Vec<f32> = vec![0.5, 0.1, 0.9, -0.3, 0.2, -0.1];

            let network = Network::from_weights(topology, weights);

            assert_eq!(network.neurons.len(), 1);
            assert_relative_eq!(network.neurons[0].bias, 0.5);
            assert_relative_eq!(network.neurons[0].coefficient, 0.1);
            assert_relative_eq!(
                network.neurons[0].weights.as_slice(),
                &[0.9, -0.3, 0.2, -0.1].as_ref()
            );
        }
    }
}
