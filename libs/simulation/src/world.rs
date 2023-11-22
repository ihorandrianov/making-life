use crate::*;

#[derive(Debug)]
struct RandomQueue<T> {
    items: Vec<T>,
}


impl<T> Index<usize> for RandomQueue<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IntoIterator for RandomQueue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> RandomQueue<T> {
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }
    
    pub fn push(&mut self, item: T, rng: &mut dyn RngCore) {
        let index = rng.gen_range(0..self.items.len() + 1);
        self.items.insert(index, item);
    }
    
    pub fn pop(&mut self, rng: &mut dyn RngCore) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.items.len());
            Some(self.items.remove(index))
        }
    }
}



#[derive(Debug)]
pub struct World {
   pub(crate) animals: Vec<Animal>,
   pub(crate) foods: Vec<Food>,
}



#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<usize>>,
    cell_size: f32,
    width: f32,
    height: f32,
}

impl Grid {
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        let cells = vec![vec![]; (width / cell_size).ceil() as usize * (height / cell_size).ceil() as usize];
        Self {
            cells,
            cell_size,
            width,
            height,
        }
    }
    
    pub fn add(&mut self, position: na::Point2<f32>, index: usize) {
        let grid_x = (position.x / self.cell_size).floor() as usize;
        let grid_y = (position.y / self.cell_size).floor() as usize;
        let cell_index = grid_x + grid_y * (self.width / self.cell_size).ceil() as usize;
        self.cells[cell_index].push(index);
    }
    
    pub fn get(&self, position: na::Point2<f32>) -> Vec<usize> {
        let x = (position.x / self.cell_size).floor() as usize;
        let y = (position.y / self.cell_size).floor() as usize;
        let cell_index = x + y * (self.width / self.cell_size).ceil() as usize;
        self.cells[cell_index].clone()
    }
    
}

impl World {
    pub fn new() -> Self {
        Self {
            animals: vec![],
            foods: vec![],
        }
    }
    pub fn random(&mut self, rng: &mut dyn RngCore) {
        let animals = self.generate_poison(rng, 1.0, 1.0, 35, 0.1).into_iter().map(|position| Animal::new(position, rng.gen(), 0.002, rng)).collect();
        let foods = self.generate_poison(rng, 1.0, 1.0, 50, 0.05).into_iter().map(|position| Food::new(position)).collect();
        
        self.animals = animals;
        self.foods = foods;
    }
   
    
    pub fn animals(&self) -> &Vec<Animal> {
        &self.animals
    }
    
    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
    
    pub(crate) fn generate_poison(&mut self, rng: &mut dyn RngCore, width: f32, height: f32, new_points_count: u32, dist: f32) -> Vec<na::Point2<f32>> {
        let cell_size = dist / SQRT_2;
        let mut grid = Grid::new(width, height, cell_size);
        
        let mut random_queue = RandomQueue::new();
        let mut points = vec![];
        
        let first_point = na::Point2::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
        
        random_queue.push(first_point, rng);
        points.push(first_point);
        grid.add(first_point, 0);
        
        while points.len() < new_points_count as usize {
            let _point = random_queue.pop(rng).unwrap_or_else(|| na::Point2::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height)));
            let new_point = na::Point2::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
            let mut min_dist = f32::INFINITY;
            for point in &points {
                let dist = (point - new_point).norm();
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            if min_dist > dist {
                random_queue.push(new_point, rng);
                points.push(new_point);
                grid.add(new_point, points.len() - 1);
            }
        }
        
        points
    }
}