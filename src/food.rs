use crate::point::Point;

use std::ops::Deref;

pub struct Food {
    food: Vec<Point>,
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

impl Deref for Food {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.food
    }
}

#[test]
fn is_at() {
    let point1 = Point::new(5, 5);
    let point2 = Point::new(6, 6);

    let mut food = Food::new();
    food.add(point1);

    assert!(!food.is_at(point2));
    assert!(food.is_at(point1));
}

#[test]
fn eat() {
    let point1 = Point::new(5, 5);
    let point2 = Point::new(6, 6);

    let mut food = Food::new();
    food.add(point1);

    food.eat(point2);
    assert_eq!(food.food.len(), 1);

    food.eat(point1);
    assert_eq!(food.food.len(), 0);
}