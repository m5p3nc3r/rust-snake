#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod point;
mod draw;
mod map;
mod snake;
mod food;
mod world;

use crate::world::{World, GameEvent, Direction};
use std::cmp;
use bevy:: {
    input::keyboard::KeyboardInput,
    app::Events,
    window::WindowResized,
    prelude::*,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

static WALL_COLOUR: Color = Color::rgb(1.0, 0.0, 0.0);
static SNAKE_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);
static FOOD_COLOUR: Color = Color::rgb(0.0, 1.0, 0.0);


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Rusty Bevy Snake".to_string(),
            width: 500.0, height: 300.0, 
            ..Default::default()
        })
        .insert_resource(World::new(WIDTH, HEIGHT))
        .insert_resource(TickTimer(Timer::from_seconds(0.5, true)))
        .add_startup_system(setup)
        .add_event::<GameEvent>()

        .add_system(window_resize_system)
        .add_system(keyboard_input_system.label("input"))

        .add_system(game_tick_system.label("tick").after("input"))
        .add_system(snake_movement_system.after("tick"))
        .add_system(food_redraw_system.after("tick"))
        .add_system(game_event_system.after("tick"))

        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )

        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeCell;

#[derive(Component)]
struct WallCell;

#[derive(Component)]
struct FoodCell;

fn create_block(colour: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: colour,
            ..Default::default()
        },
        ..Default::default()  
    }
}

const WIDTH: usize = 32;
const HEIGHT: usize = 24;

fn tile_size(windows: &Res<Windows>) -> (f32, f32, f32) {
    let window = windows.get_primary().unwrap();
    let tile_size_x = window.width() as f32 / WIDTH as f32;
    let tile_size_y = window.height() as f32 / HEIGHT as f32;

    let tile_size = cmp::min(tile_size_x as i32, tile_size_y as i32) as f32;
    let h_offset = (window.width() - (tile_size as f32 * WIDTH as f32)) / 2.;
    let v_offset = (window.height() - (tile_size as f32 * HEIGHT as f32)) / 2.;

    (tile_size, h_offset, v_offset)

}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let (tile_size, _, _) = tile_size(&windows);
    
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * tile_size, sprite_size.height * tile_size, 1.0
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let (tile_size, h_offset, v_offset) = tile_size(&windows);
    let window = windows.get_primary().unwrap();

    fn convert(pos: f32, bound_window: f32, tile_size: f32, offset: f32) -> f32 {
        pos * tile_size - (bound_window / 2.) + tile_size / 2. + offset
    }

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, tile_size as f32, h_offset),
            -convert(pos.y as f32, window.height() as f32, tile_size as f32, v_offset),
            0.0,
        );
    }
}


fn setup(mut commands: Commands, world: ResMut<World>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for point in world.snake.iter() {
        commands.spawn_bundle(
            create_block(SNAKE_COLOUR)
        )
        .insert(Position {x: point.x, y: point.y})
        .insert(Size::square(1.0))
        .insert(SnakeCell {
        });
    }

    for point in world.map.iter() {
        commands.spawn_bundle(
            create_block(WALL_COLOUR)
        )
        .insert(Position {x: point.x, y: point.y})
        .insert(Size::square(1.0))
        .insert(WallCell {
        });
    }

    for point in world.food.iter() {
        commands.spawn_bundle(
            create_block(FOOD_COLOUR)
        )
        .insert(Position {x: point.x, y: point.y})
        .insert(Size::square(1.0))
        .insert(FoodCell {
        });
    }

}
fn window_resize_system(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
//        world.dims.update_screen_size(e.width as usize, e.height as usize);
    }
}


fn keyboard_input_system(mut world: ResMut<World>, mut key_evr: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                Some(KeyCode::A) => world.last_direction = Direction::Left,
                Some(KeyCode::D) => world.last_direction = Direction::Right,
                Some(KeyCode::W) => world.last_direction = Direction::Up,
                Some(KeyCode::S) => world.last_direction = Direction::Down,

                Some(KeyCode::Left) => world.last_direction = Direction::Left,
                Some(KeyCode::Right) => world.last_direction = Direction::Right,
                Some(KeyCode::Up) => world.last_direction = Direction::Up,
                Some(KeyCode::Down) => world.last_direction = Direction::Down,
                _ => ()
            }
        }
    }
}

struct TickTimer(Timer);

fn game_tick_system(time: Res<Time>, mut timer: ResMut<TickTimer>, mut world: ResMut<World>, mut game_event: EventWriter<GameEvent>) {

    if timer.0.tick(time.delta()).just_finished() {
        // Move the game logic forward by one 'tick 
        let events = world.tick();

        // Issue any events raised
        for event in events {
            game_event.send(event);
        }
    }
}


fn game_event_system(mut commands: Commands, mut events: EventReader<GameEvent>, mut timer: ResMut<TickTimer>) {
    for event in events.iter() {
        match event {
            GameEvent::AddPoints(_score) => (),
            GameEvent::FoodEaten(_eaten_food, _new_food) => {
                // TODO: Can we explicity call the food_redraw_system here?
                //       This will stop it being called on every frame.
                //       Or, if we can reference the food by 'point', just redraw that one?

            },
            GameEvent::SnakeGrown(new_cell) => {
                commands.spawn_bundle(
                    create_block(SNAKE_COLOUR)
                )
                .insert(Position {x: new_cell.x, y: new_cell.y})
                .insert(Size::square(1.0))
                .insert(SnakeCell {
                });            
            },
            GameEvent::SpeedChanged(duration) =>{
                timer.0.set_duration(*duration);
            },
            GameEvent::GameOver => {
                println!("Game over");
            },
        }
    }
}

fn snake_movement_system(world: Res<World>,mut cell_query: Query<&mut Position, With<SnakeCell>>) {
    for (index, mut position) in cell_query.iter_mut().enumerate() {
        let point = world.snake.points[index];
    
        position.x = point.x;
        position.y = point.y;
    }
}

fn food_redraw_system(world: Res<World>, mut food_query: Query<&mut Position, With<FoodCell>>) {
    for (index, mut position) in food_query.iter_mut().enumerate() {
        let point = world.food.food[index];

        position.x = point.x;
        position.y = point.y;
    }
}

