use std::time::Duration;
use rand::seq::SliceRandom;

use crate::map::Map;
use crate::snake::Snake;
use crate::food::Food;
use crate::point::Point;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 24;


pub enum GameEvent {
    FoodEaten(Point, Option<Point>),
    SnakeGrown(Point),
    AddPoints(u32),
    SpeedChanged(Duration),
    GameOver,
}

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

#[derive(Copy, Clone, Debug)]
pub struct Dimentions {
    pub grid_width: usize,
    pub grid_height: usize,
}

pub struct World {
    pub dims: Dimentions,

    pub last_direction: Direction,
    pub tick_speed: Duration,

    pub map: Map,
    pub snake: Snake,
    pub food: Food,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            dims: Dimentions {
                grid_width: WIDTH,
                grid_height: HEIGHT,
            },

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

    pub fn tick(&mut self) -> Vec<GameEvent> {
        let mut events = vec![];
        if self.can_move_in_direction(self.last_direction) {
            events.extend(self.move_in_direction(self.last_direction));
        } else {
            events.push(GameEvent::GameOver);
        }

        events
    }

    fn can_move_in_direction(&self, direction: Direction) -> bool {
        let mut new_head = self.snake.get_head();
        new_head.move_in_direction(direction);

        let hit_self = self.snake.is_at(new_head);
        let hit_wall = self.map.wall_is_at(new_head);

        !(hit_self || hit_wall)
    }
    
    fn move_in_direction(&mut self, direction: Direction) -> Vec<GameEvent> {
        let mut ret = vec![];

        // Create a new point that will become the new head
        let mut head = self.snake.get_head();
        head.move_in_direction(direction);

        // Place it at the head of the list
        self.snake.add_head(head);

        // Pop the last point of the snake list
        if self.food.is_at(head) {
            self.food.eat(head);
            let new_food = self.add_food();
            ret.push(GameEvent::FoodEaten(head, new_food.as_ref().copied()));
            ret.push(GameEvent::SnakeGrown(self.snake.get_tail()));
            ret.push(GameEvent::AddPoints(10));
            self.tick_speed -= Duration::from_millis(10);
            ret.push(GameEvent::SpeedChanged(self.tick_speed));
        } else { 
            self.snake.remove_tail();
        }

        ret
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

        for x in 0..self.dims.grid_width-1 {
            for y in 0..self.dims.grid_height-1 {
                let point = Point::new(x as i32, y as i32);
                if self.is_free(point) {
                    free.push(point);
                }
            }
        }

        free
    }

    pub fn add_food(&mut self) -> Option<Point> {
        let free_spaces=self.get_free_spaces();

        let mut rng = rand::thread_rng();
        let food = free_spaces.choose(&mut rng);

        if let Some(f) = food { self.food.add(*f) };

        food.copied()
    }
}
