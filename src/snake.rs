use super::direction_state::{Direction, DirectionState};
use bevy::prelude::{
    default, Color, Commands, Component, Input, KeyCode, Query, Res, Sprite, SpriteBundle,
    Transform, UVec2, Vec3, Windows,
};

pub const SNAKE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

#[derive(Component)]
pub struct SnakeHead {
    direction_state: DirectionState,
    position: UVec2,
}

pub fn spawn(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead {
            direction_state: DirectionState::new(),
            position: UVec2::new(10, 10),
        });
}

pub fn direction_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_heads: Query<&mut SnakeHead>,
) {
    if let Some(mut snake_head) = snake_heads.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Left) {
            snake_head.direction_state.next_to(Direction::Left);
        } else if keyboard_input.pressed(KeyCode::Right) {
            snake_head.direction_state.next_to(Direction::Right);
        } else if keyboard_input.pressed(KeyCode::Up) {
            snake_head.direction_state.next_to(Direction::Up);
        } else if keyboard_input.pressed(KeyCode::Down) {
            snake_head.direction_state.next_to(Direction::Down);
        }
    }
}

fn normalize_coord(current_value: isize, max_value: isize) -> u32 {
    let mut result = current_value;

    if current_value < 0 {
        result = max_value - current_value;
    } else if current_value > max_value {
        result = current_value - max_value;
    }

    result as u32
}

fn normalize_number_sign(direction: Direction, value: isize) -> isize {
    value
        * match direction {
            Direction::Down | Direction::Left => -1,
            _ => 1,
        }
}

fn convert_position_to_translation(coord: f32, window_side_length: f32, grid_length: f32) -> f32 {
    let cell_size = window_side_length / grid_length;
    let cell_middle = cell_size / 2.;
    let window_middle = window_side_length / 2.;

    coord / grid_length * window_side_length - window_middle + cell_middle
}

pub fn position_iteration(mut snake_heads: Query<(&mut SnakeHead)>) {
    if let Some(mut snake_head) = snake_heads.iter_mut().next() {
        let current_direction = snake_head.direction_state.get_value();

        snake_head.position = match current_direction {
            Direction::Left | Direction::Right => UVec2::new(
                normalize_coord(
                    snake_head.position.x as isize + normalize_number_sign(current_direction, 1),
                    10,
                ),
                snake_head.position.y,
            ),
            Direction::Up | Direction::Down => UVec2::new(
                snake_head.position.x,
                normalize_coord(
                    snake_head.position.y as isize + normalize_number_sign(current_direction, 1),
                    10,
                ),
            ),
        }
    }
}

pub fn position_translation(windows: Res<Windows>, mut query: Query<(&SnakeHead, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    for (snake_head, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert_position_to_translation(snake_head.position.x as f32, window.width(), 10.),
            convert_position_to_translation(snake_head.position.y as f32, window.height(), 10.),
            0.,
        );
    }
}
