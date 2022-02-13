
use crate::point::Point;

pub struct Snake {
    pub points: Vec<Point>,
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
}
