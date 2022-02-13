use crate::point::Point;
use line_drawing::Bresenham;

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

    fn line(&mut self, start: Point, end: Point) {
        let p1 = (start.x as i32, start.y as i32);
        let p2 = (end.x as i32, end.y as i32);
    
        for (x, y) in Bresenham::new(p1, p2) {
            self.walls.push(Point::new(x, y));
        }
    }

    fn rect(&mut self, p0: Point, p3: Point) {
        let p1 = Point::new(p3.x, p0.y);
        let p2 = Point::new(p0.x, p3.y);

        self.line(p0, p1);
        self.line(p1, p3);
        self.line(p3, p2);
        self.line(p2, p0);
    }


}

impl Deref for Map {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.walls
    }
}