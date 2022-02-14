use crate::point::Point;
use crate::draw::{Rectangle};

use std::ops::Deref;

pub struct Map {
    walls: Vec<Point>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Self {
            walls: vec![],
        };

        map.rect(Point::new(0,0), Point::new(width-1, height-1));

        map
    }

    pub fn wall_is_at(&self, point:Point) -> bool {
        self.walls.iter().any( |&p| p == point)
    }

    fn rect(&mut self, p0: Point, p3: Point) {
        for point in Rectangle::new(p0, p3) {
            self.walls.push(point);
        }
    }


}

impl Deref for Map {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.walls
    }
}