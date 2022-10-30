use super::direction_state::{Direction, DirectionState};
use bevy::{
    math::Vec3,
    prelude::{
        default, Color, Commands, Component, Input, KeyCode, Query, Res, Sprite, SpriteBundle,
        Transform,
    },
};

pub const SNAKE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

#[derive(Component)]
pub struct SnakeHead {
    direction_state: DirectionState,
}

pub fn spawn(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead {
            direction_state: DirectionState::new(),
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

pub fn position_system(mut snake_position: Query<(&SnakeHead, &mut Transform)>) {
    if let Some((snake_head, mut transform)) = snake_position.iter_mut().next() {
        match snake_head.direction_state.get_value() {
            Direction::Up => {
                transform.translation.y += 2.;
            }
            Direction::Down => {
                transform.translation.y -= 2.;
            }
            Direction::Right => {
                transform.translation.x += 2.;
            }
            Direction::Left => {
                transform.translation.x -= 2.;
            }
        }
    }
}
