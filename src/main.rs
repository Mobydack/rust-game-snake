mod area;
mod direction;
mod snake;

use crate::area::resources::Area;
use bevy::time::FixedTimestep;
use bevy::{prelude::*, DefaultPlugins};

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum AreaLabel {
    Init,
    Render,
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(area::resources::AreaSettings {
            gap: 10.,
            grid_size: 10,
            background_color: Color::rgb(0., 0., 0.),
        })
        .init_resource::<Area>()
        .add_state(direction::states::Direction::Right)
        .add_startup_system(area::systems::fill_area.label(AreaLabel::Init))
        .add_startup_system(
            area::systems::render
                .label(AreaLabel::Render)
                .after(AreaLabel::Init),
        )
        .add_startup_system(snake::systems::spawn_head_segment.after(AreaLabel::Render))
        .add_system(area::systems::convert_positions_to_coord.after(AreaLabel::Render))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(snake::systems::increment_snake_head_position),
        )
        .add_system(direction::systems::change_direction.after(AreaLabel::Render))
        .add_plugins(DefaultPlugins)
        .run();
}
