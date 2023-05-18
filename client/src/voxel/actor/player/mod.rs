use super::Stats;
use crate::{
    voxel::{
        animation::{AnimationController, Animations},
        loading::MyAssets,
    },
    GameState,
};
use bevy::{core_pipeline::fxaa::Fxaa, prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, CollidingEntities, CollisionGroups, GravityScale, Group,
    KinematicCharacterController, LockedAxes, RigidBody,
};
use std::f32::consts::PI;

pub mod player_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)))
            .add_plugin(player_controller::PlayerControllerPlugin);
    }
}

fn setup(mut cmds: Commands) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

/// Marker component for player body.
#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Head;

#[derive(Component, Debug, Clone, Copy)]
pub enum CameraMode {
    FirstPerson,
    ThirdPersonForward,
}

impl CameraMode {
    const fn next(self) -> Self {
        match self {
            Self::FirstPerson => Self::ThirdPersonForward,
            Self::ThirdPersonForward => Self::FirstPerson,
        }
    }
    fn translation(self) -> Vec3 {
        match self {
            Self::FirstPerson => Vec3::ZERO,
            Self::ThirdPersonForward => Vec3::Z * -5.0,
        }
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    pub colliding_entities: CollidingEntities,

    pub gravity: GravityScale,
    pub controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    pub density: ColliderMassProperties,
    pub rotation_constraints: LockedAxes,
    pub collision_groups: CollisionGroups,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            gravity: GravityScale(0.0),
            controller: KinematicCharacterController {
                translation: Some(Vec3::new(1.0, 1.0, 1.0)),
                ..default()
            },
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            collision_groups: CollisionGroups::new(
                Group::GROUP_1,
                Group::from_bits_truncate(Group::GROUP_2.bits()),
            ),
            colliding_entities: CollidingEntities::default(),
            density: ColliderMassProperties::Density(1.0),
        }
    }
}
