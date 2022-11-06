use bevy::prelude::*;

pub struct AreaSettings {
    pub grid_size: u8,
    pub gap: f32,
    pub background_color: Color,
}

#[derive(Default)]
pub struct Area {
    pub side_length: f32,
    pub cell_length: f32,
}
