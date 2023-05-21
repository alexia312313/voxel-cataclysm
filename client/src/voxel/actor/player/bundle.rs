use super::{CameraMode, Head};
use crate::voxel::{animation::AnimationController, Stats};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Collider, CollidingEntities, GravityScale, KinematicCharacterController, LockedAxes, RigidBody,
};
use std::f32::consts::PI;

#[derive(Bundle)]
pub struct BasePlayerBundle {
    pub colliding_entities: CollidingEntities,
    pub gravity: GravityScale,
    pub stats: Stats,
    pub visibility: VisibilityBundle,
    pub controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    //pub density: ColliderMassProperties,
    pub rotation_constraints: LockedAxes,
    //pub collision_groups: CollisionGroups,
    pub animation_controller: AnimationController,
}

impl Default for BasePlayerBundle {
    fn default() -> Self {
        Self {
            // animation
            animation_controller: AnimationController { done: false },
            // stats
            stats: Stats {
                hp: 100,
                max_hp: 100,
                attack: 5,
                speed: 10.0,
                score: 0,
            },
            // visibility
            visibility: VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
            // physics
            rigid_body: RigidBody::Dynamic,
            gravity: GravityScale(2.0),
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            colliding_entities: CollidingEntities::default(),
            //density: ColliderMassProperties::Density(1.0),
            controller: KinematicCharacterController {
                translation: Some(Vec3::new(1.0, 1.0, 1.0)),
                ..default()
            },
            /*
            collision_groups: CollisionGroups::new(
                Group::GROUP_1,
                Group::from_bits_truncate(Group::GROUP_2.bits()),
            ),
            */
        }
    }
}

#[derive(Bundle)]
pub struct MyCamera3dBundle {
    camera: Camera3dBundle,
    mode: CameraMode,
}
impl Default for MyCamera3dBundle {
    fn default() -> Self {
        Self {
            camera: Camera3dBundle {
                projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                    fov: PI / 2.,
                    far: 2048.0,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                    .looking_to(Vec3::Z, Vec3::Y),
                camera: Camera {
                    order: (1),
                    ..default()
                },
                ..Default::default()
            },
            mode: CameraMode::ThirdPersonForward,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerColliderBundle {
    collider: Collider,
    transform: TransformBundle,
}
impl Default for PlayerColliderBundle {
    fn default() -> Self {
        Self {
            collider: Collider::capsule_y(0.5, 0.5),
            transform: TransformBundle {
                local: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerHeadBundle {
    head: Head,
    transform: TransformBundle,
}
impl Default for PlayerHeadBundle {
    fn default() -> Self {
        Self {
            head: Head,
            transform: TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, 0.9, 0.0))
                    .looking_to(Vec3::Z, Vec3::Y),
                ..default()
            },
        }
    }
}
