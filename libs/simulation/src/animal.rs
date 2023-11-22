use crate::*;
use crate::eye::Eye;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}

impl Animal {
    pub(crate) fn new(position: na::Point2<f32>, rotation: na::Rotation2<f32>, speed: f32, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self {
            position,
            rotation,
            speed,
            eye,
            brain,
            satiation: 0,
        }
    }
    
    
    pub fn position(&self) -> &na::Point2<f32> {
        &self.position
    }
    
    pub fn rotation(&self) -> &na::Rotation2<f32> {
        &self.rotation
    }
    
    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }
    
    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore, position: na::Point2<f32>) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);
        Self {
            position: position,
            rotation: rng.gen(),
            speed: 0.01,
            eye,
            brain,
            satiation: 0,
        }
    }
}