#![feature(type_alias_impl_trait)]
use rand::{RngCore, Rng};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;
use rand::SeedableRng;
use std::ops::Index;



pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S> where S: SelectionMethod {
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static, mutation_method: impl MutationMethod + 'static) -> Self {
        Self { selection_method, crossover_method: Box::new(crossover_method), mutation_method: Box::new(mutation_method) }
    }
    
    pub fn evolve<I>(
        &self,
        rng: &mut dyn RngCore,
        population: &[I]
    ) -> Vec<I> where I: Individual {
        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect()
    }
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}


pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub trait CrossoverMethod {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome;
}

pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self {}
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome {
        let genes = parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(gene_a, gene_b)| {
                if rng.gen_bool(0.5) {
                    *gene_a
                } else {
                    *gene_b
                }
            })
            .collect();
        Chromosome::new(genes)
    }
}

impl Default for UniformCrossover {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self {}
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I],
    ) -> &'a I 
    where 
        I: Individual,
        {
            population
                .choose_weighted(rng, |individual| individual.fitness())
                .expect("population must not be empty")
        }
}


impl Default for RouletteWheelSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn new(genes: Vec<f32>) -> Self {
        Self { genes }
    }
    
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
    pub fn is_empty(&self) -> bool {
        self.genes.is_empty()
    }
    
    
}

impl Index<usize> for Chromosome {
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}


impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl<'a> IntoIterator for &'a Chromosome {
    type Item = &'a f32;
    type IntoIter = impl Iterator<Item = &'a f32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.genes.iter()
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = f32>,
        {
            Self {
                genes: iter.into_iter().collect(),
            }
        }
}



pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}


#[derive(Clone, Debug)]
pub struct GaussianMutation {
   chance: f32,
   coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            
            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}



#[cfg(test)]
#[derive(Clone, Debug)]
pub struct MockIndividual {
    fitness: f32,
}

#[cfg(test)]
impl MockIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for MockIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteWheelSelection::new();
        let population = vec![
            MockIndividual::new(1.0),
            MockIndividual::new(2.0),
            MockIndividual::new(3.0),
            MockIndividual::new(4.0),
            MockIndividual::new(5.0),
        ];
        
       let expected_histogram = maplit::btreemap! {
        1 => 72,
        2 => 130,
        3 => 202,
        4 => 278,
        5 => 318,
       };
       
       let actual_histogram: BTreeMap<i32, _> = (0..1000)
        .map(|_| method.select(&mut rng, &population))
        .fold(Default::default(), |mut histogram, individual| {
            *histogram.entry(individual.fitness() as _).or_default() += 1;
            histogram
        });
        
        assert_eq!(actual_histogram, expected_histogram);
        
    }
}

