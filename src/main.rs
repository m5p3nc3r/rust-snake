#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod point;
mod snake;
mod food;

use std::cmp::min;
use log::error;
use instant::Instant;
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, StartCause, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use line_drawing::Bresenham;
use rand::seq::SliceRandom;

use crate::snake::Snake;
use crate::food::Food;
use crate::point::Point;

const WIDTH: usize = 32;
const HEIGHT: usize = 24;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
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



struct World {
    width: usize,
    height: usize,

    last_direction: Direction,
    tick_speed: Duration,

    snake: Snake,
    food: Food,
}

impl World {
    fn new() -> Self {
        let mut world = Self {
            width: WIDTH,
            height: HEIGHT,

            last_direction: Direction::Right,
            tick_speed: Duration::from_millis(500),

            snake: Snake::new(),
            food: Food::new()
        };

        world.add_food();
        world.add_food();
        world.add_food();

        world
    }

    fn tick(&mut self) {
        if self.can_move_in_direction(self.last_direction) {
            self.move_in_direction(self.last_direction);
        } else {
            println!("GAME OVER");
        }
    }

    fn hit_wall(&self, point: Point) -> bool {
        point.x == 0 || point.x == (self.width - 1) as i32 ||
        point.y == 0 || point.y == (self.height - 1) as i32
    }

    fn can_move_in_direction(&self, direction: Direction) -> bool {
        let mut new_head = self.snake.get_head();
        new_head.move_in_direction(direction);

        let hit_self = self.snake.is_at(new_head);
        let hit_wall = self.hit_wall(new_head);

        !(hit_self || hit_wall)
    }
    
    fn move_in_direction(&mut self, direction: Direction) {
        // Create a new point that will become the new head
        let mut head = self.snake.get_head();
        head.move_in_direction(direction);

        // Place it at the head of the list
        self.snake.points.insert(0,head);

        // Pop the last point of the snake list
        if self.food.is_at(head) {
            self.food.eat(head);
            self.add_food();
            self.tick_speed -= Duration::from_millis(10);
        } else { 
            self.snake.points.pop();
        }
    }


    fn is_free(&self, point: Point) -> bool {
        let hit_snake = self.snake.is_at(point);
        let hit_wall = self.hit_wall(point);
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

    fn add_food(&mut self) {
        let free_spaces=self.get_free_spaces();

        let mut rng = rand::thread_rng();
        let food = free_spaces.choose(&mut rng);

        if let Some(f) = food { self.food.add(*f) };
    }



    // TODO:: Move these into a rendering class
    fn point(&self, frame: &mut [u8], point: &Point, colour: [u8; 4]) {
        let x = min(point.x as usize, self.width - 1);
        let y = min(point.y as usize, self.height - 1);
        let i = x * 4 + y * WIDTH * 4;

        frame[i..i + 4].copy_from_slice(&colour);
    }

    fn line(&self, frame: &mut [u8], start: Point, end: Point, colour: [u8; 4]) {
        let p1 = (start.x as i32, start.y as i32);
        let p2 = (end.x as i32, end.y as i32);
    
        for (x, y) in Bresenham::new(p1, p2) {
            self.point(frame, &Point::new(x, y), colour);
        }
    }

    fn rect(&self, frame: &mut[u8], p0: Point, p3: Point, colour: [u8; 4]) {
        let p1 = Point::new(p3.x, p0.y);
        let p2 = Point::new(p0.x, p3.y);

        self.line(frame, p0, p1, colour);
        self.line(frame, p1, p3, colour);
        self.line(frame, p3, p2, colour);
        self.line(frame, p2, p0, colour);
    }

    fn draw(&self, frame: &mut [u8] ) {
        let black = [0x00, 0x00, 0x00, 0xff];
        let red = [0xff, 0x00, 0x00, 0xff];
        let white = [0xff, 0xff, 0xff, 0xff];
        let green = [0x00, 0xff, 0x00, 0xff];

        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&black);
        }

        let x0 = Point::new(0,0);
        let x1= Point::new((self.width-1) as i32,(self.height-1) as i32);

        self.rect(frame, x0, x1, red);

        for point in &self.snake.points {
            self.point(frame, point, white);
        }

        for food in &self.food.food {
            self.point(frame,food,green);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let inner_size = LogicalSize::new((WIDTH*20) as f64, (HEIGHT*20) as f64);
        WindowBuilder::new()
            .with_title("Rusty Snake")
            .with_inner_size(inner_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface).unwrap()
    };

    pixels.set_clear_color(Color::BLACK);

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) =>  {
                world.draw(pixels.get_frame());

                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            },

            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + world.tick_speed )
            },
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + world.tick_speed );
                world.tick();
                window.request_redraw();
            },

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    device_id: _, input: kin, is_synthetic: _
                } => {
                    if kin.state == ElementState::Pressed {
                        match kin.virtual_keycode {
                            Some(VirtualKeyCode::A) => world.last_direction=Direction::Left,
                            Some(VirtualKeyCode::S) => world.last_direction=Direction::Down,
                            Some(VirtualKeyCode::W) => world.last_direction=Direction::Up,
                            Some(VirtualKeyCode::D) => world.last_direction=Direction::Right,

                            Some(VirtualKeyCode::P) => world.add_food(),
                            _ => ()
                        }
                    }
                },
                WindowEvent::Resized(size) =>  {
                    pixels.resize_surface(size.width, size.height);
                    window.request_redraw();
                    
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },

            // Event::DeviceEvent { event, .. } => match event {
            //     _ => {
            //         //println!("Device event {:?}", event);
            //     }
            // }
        
            _ => (),
        }
    });
}

