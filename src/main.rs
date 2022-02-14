#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod point;
mod draw;
mod map;
mod snake;
mod food;
mod world;

use log::error;
use instant::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, StartCause, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture, wgpu::Color};
use crate::world::{World, Direction, WIDTH, HEIGHT};



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

                            Some(VirtualKeyCode::Q) => *control_flow = ControlFlow::Exit,
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

