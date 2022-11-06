use crate::direction::states::Direction;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position(pub UVec2);

impl Position {
    fn get_incremented_infinity_coord(&self, value: i32, increment: i32, limit: i32) -> u32 {
        let next_value = value + increment;
        let result = if next_value < 1 {
            limit
        } else if next_value > limit {
            1
        } else {
            next_value
        };

        result as u32
    }

    pub fn increment(&mut self, direction: Direction, limit: i32) {
        let increment: i32 = match direction {
            Direction::Up | Direction::Left => -1,
            _ => 1,
        };

        match direction {
            Direction::Left | Direction::Right => {
                self.0.x = self.get_incremented_infinity_coord(self.0.x as i32, increment, limit);
            }
            _ => {
                self.0.y = self.get_incremented_infinity_coord(self.0.y as i32, increment, limit);
            }
        }
    }
}
