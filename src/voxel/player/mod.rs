use bevy::prelude::*;

use std::f32::consts::PI;

use bevy::core_pipeline::fxaa::Fxaa;

pub mod player_anim;
pub mod player_mov;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(player_anim::PlayerAnimationsHandlePlugin)
            .add_plugin(player_mov::PlayerControllerPlugin)
            //.add_plugin(animation_link::AnimationLinkPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut cmds: Commands, asset: Res<AssetServer>) {
    // Spawn the player
    cmds.spawn(Camera3dBundle {
        projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.,
            far: 2048.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(2.0, 160.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PlayerController::default())
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());

    // Spawn the sky light
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

#[derive(Default, Component, Debug)]
pub struct PlayerController {
    yaw: f32,
    pitch: f32,
    cursor_locked: bool,
    prev_xyz: Vec3,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, SystemSet)]
/// Systems related to player controls.
pub struct PlayerControllerSet;
