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

fn setup(mut cmds: Commands, _my_assets: Res<MyAssets>) {
    let mut map = HashMap::new();
    map.insert("walk".to_string(), _my_assets.player_animation_walk.clone());
    map.insert("hit".to_string(), _my_assets.player_animation_hit.clone());

    cmds.spawn((
        Player,
        Stats {
            hp: 100,
            max_hp: 100,
            attack: 5,
            speed: 10.0,
        },
        ColliderBundle::default(),
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(2.0, 160.0, 2.0).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
    ))
    .with_children(|player| {
        player.spawn(Body).insert(SceneBundle {
            scene: _my_assets.player.clone(),
            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
            ..default()
        });
        player
            .spawn((
                Head,
                TransformBundle {
                    // head is 1.8m above feet
                    local: Transform::from_translation(Vec3::new(0.0, 0.9, 0.0))
                        .looking_to(Vec3::Z, Vec3::Y),
                    ..default()
                },
            ))
            .with_children(|head| {
                // spawn camera as a child of head
                head.spawn(Camera3dBundle {
                    projection: bevy::render::camera::Projection::Perspective(
                        PerspectiveProjection {
                            fov: PI / 2.,
                            far: 2048.0,
                            ..Default::default()
                        },
                    ),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                        .looking_to(Vec3::Z, Vec3::Y),
                    ..Default::default()
                })
                .insert(CameraMode::ThirdPersonForward);
            });
    })
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default())
    .insert(AnimationController { done: false })
    .insert(Animations(map));

    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

#[derive(Component)]
pub struct Player;

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
    pub collider: Collider,
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
            collider: Collider::capsule_y(2., 1.5),
            rigid_body: RigidBody::Dynamic,
            gravity: GravityScale(1.0),
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
