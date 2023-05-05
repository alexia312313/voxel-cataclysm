#![allow(
    clippy::type_complexity,
    clippy::manual_clamp,
    clippy::module_inception
)]

use bevy::prelude::*;
use bevy::window::WindowMode;

mod debug;
mod voxel;

fn main() {
    let mut app = App::default();
    app
        //
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "vx_bevy".into(),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(voxel::VoxelWorldPlugin)
        .add_plugin(debug::DebugUIPlugins)
        .add_plugin(voxel::PlayerPlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Game,
}
