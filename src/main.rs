mod direction_state;
mod snake;

use bevy::time::FixedTimestep;
use bevy::{prelude::*, DefaultPlugins};

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn)
        .add_system(snake::direction_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(snake::position_iteration),
        )
        .add_system(snake::position_translation.after(snake::position_iteration))
        .add_plugins(DefaultPlugins)
        .run();
}
