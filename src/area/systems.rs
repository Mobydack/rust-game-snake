use super::{
    components::Position,
    resources::{Area, AreaSettings},
};
use bevy::prelude::*;

pub fn fill_area(area_settings: Res<AreaSettings>, mut area: ResMut<Area>, windows: Res<Windows>) {
    let current_window = windows.get_primary().unwrap();
    let side_length = current_window.width().min(current_window.height()) - area_settings.gap;

    area.side_length = side_length;
    area.cell_length = side_length / area_settings.grid_size as f32;
}

pub fn render(area_settings: Res<AreaSettings>, area: Res<Area>, mut commands: Commands) {
    let _ = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: area_settings.background_color,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(area.side_length, area.side_length, default()),
                ..default()
            },
            ..default()
        })
        .id();
}

pub fn convert_positions_to_coord(area: Res<Area>, mut query: Query<(&Position, &mut Transform)>) {
    for (Position(position), mut transform) in &mut query {
        let UVec2 { x, y } = *position;

        transform.translation = Vec3::new(
            area.side_length / -2. - area.cell_length / 2. + area.cell_length * (x as f32),
            area.side_length / 2. - area.cell_length / 2. - area.cell_length * (y as f32 - 1.),
            transform.translation.z,
        );
    }
}
