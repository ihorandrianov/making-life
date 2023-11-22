use crate::*;




#[derive(Debug, Clone)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
}

impl Food {
    pub fn new(position: na::Point2<f32>) -> Self {
        Self {
            position,
        }
    }
    
    pub fn position(&self) -> &na::Point2<f32> {
        &self.position
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Food {}
