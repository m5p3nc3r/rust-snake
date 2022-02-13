
use crate::point::Point;

use std::ops::Deref;

pub struct Snake {
    points: Vec<Point>,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            points: vec![Point::new(10,10), Point::new(9, 10), Point::new(8,10), Point::new(7,10)],
        }
    }

    pub fn is_at(&self, point: Point) -> bool {
        self.points.iter().any( |&p| p == point)
    }

    pub fn get_head(&self) -> Point {
        self.points[0]
    }

    pub fn add_head(&mut self, point: Point) {
        self.points.insert(0,point);
    }

    pub fn remove_tail(&mut self) {
        self.points.pop();
    }
}

impl Deref for Snake {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.points
    }
}
