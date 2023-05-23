use bevy::prelude::*;

use crate::{voxel::loading::MyAssets, GameState};

pub mod bundle;
pub mod controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(equip_weapon.in_set(OnUpdate(GameState::Game)))
            .add_plugin(controller::PlayerControllerPlugin);
    }
}

pub fn equip_weapon(
    mut queries: ParamSet<(Query<Entity, With<Body>>, Query<Entity, With<Weapon>>)>,
    children_query: Query<&Children>,
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
) {
    if queries.p1().iter().count() > 0 {
        return;
    }

    for entity in queries.p0().iter() {
        let mut index: u16 = 0;
        for child in children_query.iter_descendants(entity) {
            index += 1;
            if index == 4 {
                commands.entity(child).with_children(|parent| {
                    parent.spawn((
                        SceneBundle {
                            scene: _my_assets.sword.clone_weak(),
                            transform: Transform::from_xyz(0.0, -0.8, -0.2)
                                .with_rotation(Quat::from_rotation_y(-0.2)),
                            ..default()
                        },
                        Weapon,
                    ));
                });
            }
        }
    }
}

/// Marker component for player body.
#[derive(Component)]
pub struct Body;

/// Marker component for player body.
#[derive(Component)]
pub struct Weapon;

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
