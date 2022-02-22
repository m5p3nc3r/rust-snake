
use crate::point::Point;

use std::ops::Deref;

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

    pub fn add_head(&mut self, point: Point) {
        self.points.insert(0,point);
    }

    pub fn get_tail(&self) -> Point {
        self.points[self.points.len()-1]
    }

    pub fn remove_tail(&mut self) -> Option<Point> {
        self.points.pop()
    }
}

impl Deref for Snake {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

#[test]
fn test_get_head() {
    let snake = Snake::new();
    let head = snake.get_head();
    assert_eq!(head, Point::new(10, 10))
}

#[test]
fn test_add_head() {
    let mut snake = Snake::new();
    let head = Point::new(11, 10);
    snake.add_head(head);
    assert_eq!(snake.points.len(), 5);
    assert_eq!(snake.points[0], head);
    assert_eq!(snake.get_head(), head);
}

#[test]
fn remove_tail() {
    let mut snake = Snake::new();
    let tail = snake.remove_tail();
    assert_eq!(snake.points.len(), 3);
    assert_eq!(tail, Some(Point::new(7, 10)));
}

#[test]
fn test_iter() {
    let snake = Snake::new();
    let mut iter = snake.iter();
    assert_eq!(iter.next(), Some(&Point::new(10,10)));
    assert_eq!(iter.next(), Some(&Point::new(9,10)));
    assert_eq!(iter.next(), Some(&Point::new(8,10)));
    assert_eq!(iter.next(), Some(&Point::new(7,10)));
    assert_eq!(iter.next(), None);
}
