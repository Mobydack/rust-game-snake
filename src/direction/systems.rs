use super::states::Direction;
use bevy::prelude::*;

pub fn change_direction(
    keys: ResMut<Input<KeyCode>>,
    mut direction_state: ResMut<State<Direction>>,
) {
    let current_direction = *direction_state.current();
    let next_direction: Direction = Direction::safe_next(
        current_direction,
        if keys.pressed(KeyCode::Down) {
            Direction::Down
        } else if keys.pressed(KeyCode::Up) {
            Direction::Up
        } else if keys.pressed(KeyCode::Left) {
            Direction::Left
        } else if keys.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            current_direction
        },
    );

    if next_direction == current_direction {
        return;
    }

    direction_state.replace(next_direction).unwrap();
}
