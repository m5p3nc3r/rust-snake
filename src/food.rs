use crate::point::Point;

pub struct Food {
    pub food: Vec<Point>,
}

impl Food {
    pub fn new() -> Self {
        Self {
            food: vec![]
        }
    }

    pub fn is_at(&self, point: Point) -> bool {
        self.food.iter().any( |&p| p == point)
    }

    pub fn eat(&mut self, point: Point) {
        let index = self.food.iter().position(|f| *f==point);
        if let Some(i) = index { self.food.remove(i); };
    }

    pub fn add(&mut self, food: Point) {
        self.food.push(food);
    }
}
