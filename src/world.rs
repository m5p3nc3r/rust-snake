use std::cmp::min;
use std::time::Duration;
use rand::seq::SliceRandom;

use crate::map::Map;
use crate::snake::Snake;
use crate::food::Food;
use crate::point::Point;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 24;


trait Move {
    fn move_in_direction(&mut self, direction: Direction);
}

impl Move for Point {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}


pub struct World {
    width: usize,
    height: usize,

    pub last_direction: Direction,
    pub tick_speed: Duration,

    map: Map,
    snake: Snake,
    food: Food,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            width: WIDTH,
            height: HEIGHT,

            last_direction: Direction::Right,
            tick_speed: Duration::from_millis(500),

            map: Map::new(WIDTH as i32, HEIGHT as i32),
            snake: Snake::new(),
            food: Food::new()
        };

        world.add_food();
        world.add_food();
        world.add_food();

        world
    }

    pub fn tick(&mut self) {
        if self.can_move_in_direction(self.last_direction) {
            self.move_in_direction(self.last_direction);
        } else {
            println!("GAME OVER");
        }
    }

    fn can_move_in_direction(&self, direction: Direction) -> bool {
        let mut new_head = self.snake.get_head();
        new_head.move_in_direction(direction);

        let hit_self = self.snake.is_at(new_head);
        let hit_wall = self.map.wall_is_at(new_head);

        !(hit_self || hit_wall)
    }
    
    fn move_in_direction(&mut self, direction: Direction) {
        // Create a new point that will become the new head
        let mut head = self.snake.get_head();
        head.move_in_direction(direction);

        // Place it at the head of the list
        self.snake.add_head(head);

        // Pop the last point of the snake list
        if self.food.is_at(head) {
            self.food.eat(head);
            self.add_food();
            self.tick_speed -= Duration::from_millis(10);
        } else { 
            self.snake.remove_tail();
        }
    }


    fn is_free(&self, point: Point) -> bool {
        let hit_snake = self.snake.is_at(point);
        let hit_wall = self.map.wall_is_at(point);
        let hit_food = self.food.is_at(point);

        !hit_snake && !hit_wall && !hit_food
    }

    //
    fn get_free_spaces(&self) -> Vec<Point> {
        let mut free=Vec::new();

        for x in 0..self.width-1 {
            for y in 0..self.height-1 {
                let point = Point::new(x as i32, y as i32);
                if self.is_free(point) {
                    free.push(point);
                }
            }
        }

        free
    }

    pub fn add_food(&mut self) {
        let free_spaces=self.get_free_spaces();

        let mut rng = rand::thread_rng();
        let food = free_spaces.choose(&mut rng);

        if let Some(f) = food { self.food.add(*f) };
    }



    // TODO:: Move these into a rendering class
    fn point(&self, frame: &mut [u8], point: &Point, colour: [u8; 4]) {
        let x = min(point.x as usize, self.width - 1);
        let y = min(point.y as usize, self.height - 1);
        let i = x * 4 + y * self.width/*WIDTH*/ * 4;

        frame[i..i + 4].copy_from_slice(&colour);
    }

    pub fn draw(&self, frame: &mut [u8] ) {
        let black = [0x00, 0x00, 0x00, 0xff];
        let red = [0xff, 0x00, 0x00, 0xff];
        let white = [0xff, 0xff, 0xff, 0xff];
        let green = [0x00, 0xff, 0x00, 0xff];

        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&black);
        }

        for point in self.map.iter() {
            self.point(frame, point, red);
        }

        for point in self.snake.iter() {
            self.point(frame, point, white);
        }

        for food in self.food.iter() {
            self.point(frame,food,green);
        }
    }
}
