use rand::{SeedableRng, RngCore, thread_rng, Rng};
use std::iter::once;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}


#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore,layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons, rng))
            .collect();
            
        Self { layers }
    }
    
    pub fn propagate(&self, input: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(input, |acc, layer| layer.propagate(acc))
    }
    
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        
        
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
            
    }
    
    pub fn from_weights(weights: impl IntoIterator<Item = f32>, layers: &[LayerTopology]) -> Self {
        let mut weights = weights.into_iter();
        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();
            
        if weights.next().is_some() {
            panic!("Too many weights");
        }
        Self { layers }
    }
}

impl Layer {
    
    fn random(input_neurons: usize, neurons: usize, rng: &mut dyn RngCore) -> Self {
        
        let neurons = (0..neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();
        Self { neurons }
    }
    
    pub fn propagate(&self, input: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&input))
            .collect()
    }
    
    pub fn from_weights(input_neurons: usize, neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let neurons = (0..neurons)
            .map(|_| Neuron::from_weights(input_neurons, weights))
            .collect();
        Self { neurons }
    }
}

impl Neuron {
    fn random(rng: &mut dyn RngCore ,input_neurons: usize) -> Self {
        
        let weights = (0..input_neurons)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
        let bias = rng.gen_range(-1.0..1.0);
        Self { weights, bias }
    }
    fn propagate(&self, input: &[f32]) -> f32 {
        let output = input.iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }
    
    fn from_weights(input_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().unwrap();
        let weights: Vec<f32> = (0..input_neurons)
            .map(|_| weights.next().unwrap())
            .collect();
        
        Self { weights, bias }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;


    #[test]
    fn creates_random_items() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 3);
        
        assert_eq!(neuron.bias, 0.26284885);
        assert_eq!(neuron.weights, &[-0.6255188, 0.67383933, 0.81812596]);
    }
    
    #[test]
    fn relu_function_works() {
        let neuron = Neuron {
            weights: vec![1.0, 1.0, 1.0],
            bias: 0.0,
        };
        
        assert_eq!(neuron.propagate(&[1.0, 1.0, 1.0]), 3.0);
    }
    
    #[test]
    fn relu_function_works_with_negative() {
        let neuron = Neuron {
            weights: vec![1.0, 1.0, 1.0],
            bias: 0.0,
        };
        
        assert_eq!(neuron.propagate(&[-1.0, -1.0, -1.0]), 0.0);
    }
    
    #[test]
    fn relu_function_works_with_bias() {
        let neuron = Neuron {
            weights: vec![1.0, 1.0, 1.0],
            bias: 1.0,
        };
        
        assert_eq!(neuron.propagate(&[1.0, 1.0, 1.0]), 4.0);
    }
    
}
