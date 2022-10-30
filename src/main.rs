mod direction_state;
mod snake;

use bevy::{
    prelude::{App, Camera2dBundle, Commands},
    DefaultPlugins,
};

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn)
        .add_system(snake::direction_system)
        .add_system(snake::position_system)
        .add_plugins(DefaultPlugins)
        .run();
}
