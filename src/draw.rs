use crate::point::Point;
use line_drawing::Bresenham;

pub struct Line {
    bresenham: Bresenham<i32>
}

impl Line {
    pub fn new(p0: Point, p1: Point) -> Self {
        Self {
            bresenham: Bresenham::new((p0.x, p0.y), (p1.x, p1.y)),
        }
    }
}


impl Iterator for Line {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let point = self.bresenham.next();

        point.map(|point| Point::new(point.0,point.1))
    }
}

pub struct Rectangle {
    line0: Line,
    line1: Line,
    line2: Line,
    line3: Line,
}

impl Rectangle {
    pub fn new(p0: Point, p3: Point) -> Self {
        let p1 = Point::new(p3.x, p0.y);
        let p2 = Point::new(p0.x, p3.y);


        Self {
            line0: Line::new(p0, p1),
            line1: Line::new(p1, p3),
            line2: Line::new(p3, p2),
            line3: Line::new(p2, p0),
        }
    }
}

impl Iterator for Rectangle {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut point = self.line0.next();
        if point.is_none() { point = self.line1.next() }
        if point.is_none() { point = self.line2.next() }
        if point.is_none() { point = self.line3.next() }

        point
    }
}