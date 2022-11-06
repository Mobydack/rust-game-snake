use crate::area::{
    components::Position,
    resources::{Area, AreaSettings},
};
use crate::direction::states::Direction;

use bevy::prelude::*;

pub fn spawn_head_segment(
    mut commands: Commands,
    area: Res<Area>,
    area_settings: Res<AreaSettings>,
) {
    let _ = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(area.cell_length, area.cell_length, default()),
                ..default()
            },
            ..default()
        })
        .insert(Position(UVec2::new(
            (area_settings.grid_size / 2) as u32,
            (area_settings.grid_size / 2) as u32,
        )))
        .id();
}

pub fn increment_snake_head_position(
    area_settings: Res<AreaSettings>,
    direction_state: Res<State<Direction>>,
    mut positions: Query<&mut Position>,
) {
    for mut position in &mut positions {
        position.increment(*direction_state.current(), area_settings.grid_size as i32);
    }
}
