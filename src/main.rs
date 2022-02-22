#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod point;
mod draw;
mod map;
mod snake;
mod food;
mod world;

use crate::world::{World, GameEvent, Direction, Dimentions};

use bevy:: {
    core::FixedTimestep,
    input::keyboard::KeyboardInput,
    app::Events,
    window::WindowResized,
    prelude::*,
};
use crate::point::Point;


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
        .insert_resource(World::new())
        .add_startup_system(setup)
        .add_event::<GameEvent>()

        .add_system(window_resize_system)
        .add_system(keyboard_input_system.label("input"))

        .add_system_set(
            SystemSet::new()
                .after("input")
                .with_run_criteria(
                    FixedTimestep::step(0.5)
                )
                .with_system(game_tick_system.label("tick"))
                .with_system(snake_movement_system.after("tick"))
                .with_system(food_redraw_system.after("tick"))
        )
        .add_system(game_event_system.after("tick"))

        .add_system(bevy::input::system::exit_on_esc_system)
        .run();

}

#[derive(Component)]
struct SnakeCell;


#[derive(Component)]
struct WallCell;

#[derive(Component)]
struct FoodCell;


fn coords(point: Point, dims: Dimentions) -> (i32, i32) {
    let v = (dims.screen_height - (dims.block_size * dims.grid_height)) / 2;
    let h = (dims.screen_width - (dims.block_size * dims.grid_width)) / 2;

    let block_size_div_2 = (dims.block_size / 2) as i32;

    let x = (point.x * dims.block_size as i32) - (dims.screen_width  as i32 / 2) + h as i32 + block_size_div_2;
    let y = -((point.y * dims.block_size as i32) - (dims.screen_height as i32 / 2) + v as i32 + block_size_div_2);

    (x, y)
}

fn create_block(point: Point, colour: Color, dims: Dimentions) -> SpriteBundle {
    let (x, y) = coords(point, dims);

    SpriteBundle {
        transform: Transform {
            scale: Vec3::new(dims.block_size as f32, dims.block_size as f32, 0.0),
            translation: Vec3::new(x as f32, y as f32, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: colour,
            ..Default::default()
        },
        ..Default::default()  
    }
}


fn setup(mut commands: Commands, mut world: ResMut<World>, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    let window = windows.get_primary().unwrap();

    world.dims.update_screen_size(window.width() as usize, window.height() as usize);

    for point in world.snake.iter() {
        commands.spawn_bundle(
            create_block(*point, SNAKE_COLOUR, world.dims)
        )
        .insert(SnakeCell {
        });
    }

    for point in world.map.iter() {
        commands.spawn_bundle(
            create_block(*point, WALL_COLOUR, world.dims)
        )
        .insert(WallCell {
        });
    }

    for point in world.food.iter() {
        commands.spawn_bundle(
            create_block(*point, FOOD_COLOUR, world.dims)
        )
        .insert(FoodCell {
        });
    }

}
fn window_resize_system(mut world: ResMut<World>, resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
        world.dims.update_screen_size(e.width as usize, e.height as usize);
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

fn game_tick_system(mut world: ResMut<World>, mut game_event: EventWriter<GameEvent>) {
    // Move the game logic forward by one 'tick'
    let events = world.tick();

    // Issue any events raised
    for event in events {
        game_event.send(event);
    }
}


fn game_event_system(mut commands: Commands, world: Res<World>, mut events: EventReader<GameEvent>) {
    for event in events.iter() {
        match event {
            GameEvent::AddPoints(_score) => (),
            GameEvent::FoodEaten(_eaten_food, _new_food) => {
                // TODO: Can we explicity call the food_redraw_system here?
                //       This will stop it being called on every frame.
                //       Or, if we can reference the food by 'point', just redraw that one?

            },
            GameEvent::SnakeGrown(new_cell) => {
                // We have eaten food - grow the snake by one cell
                commands.spawn_bundle(
                    create_block(*new_cell, SNAKE_COLOUR, world.dims)
                )
                .insert(SnakeCell {
                });
                    
              
            },
            GameEvent::SpeedChanged(duration) =>{
                println!("TODO: Implement speed change {:?}", duration);
            },
            GameEvent::GameOver => {
                println!("Game over");
            },
        }
    }
}

fn snake_movement_system(world: Res<World>,mut cell_query: Query<(&SnakeCell, &mut Transform)>) {
    for (index, (_cell, mut transform)) in cell_query.iter_mut().enumerate() {
        let point = world.snake.points[index];
        let (x, y) = coords(point, world.dims);

        let translation = &mut transform.translation;
        translation.x = x as f32;
        translation.y = y as f32;
    }
}

fn food_redraw_system(world: Res<World>, mut food_query: Query<(&FoodCell, &mut Transform)>) {
    for (index, (_food, mut transform)) in food_query.iter_mut().enumerate() {
        let point = world.food.food[index];

        let (x, y) = coords(point, world.dims);

        let translation = &mut transform.translation;
        translation.x = x as f32;
        translation.y = y as f32;
    }
}

