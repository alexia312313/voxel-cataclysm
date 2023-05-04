#![allow(
    clippy::type_complexity,
    clippy::manual_clamp,
    clippy::module_inception
)]

use bevy::prelude::*;

mod debug;
mod voxel;

fn main() {
    let mut app = App::default();
    app.add_plugins(DefaultPlugins)
        .add_plugin(voxel::VoxelWorldPlugin)
        .add_plugin(debug::DebugUIPlugins)
        .add_plugin(voxel::PlayerPlugin)
        .run();
}
