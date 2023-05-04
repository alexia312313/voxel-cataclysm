use bevy::prelude::*;

use std::f32::consts::PI;

use bevy::core_pipeline::fxaa::Fxaa;

pub mod player_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player_controller::PlayerControllerPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera3dBundle {
        projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.,
            far: 2048.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(crate::voxel::player_controller::PlayerController::default())
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());

    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}
