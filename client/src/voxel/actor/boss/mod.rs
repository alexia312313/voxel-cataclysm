use super::{Stats, mob::brain};
use crate::{
    voxel::{
        animation::{AnimationController, Animations},
        loading::MyAssets,
    },
    GameState,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, LockedAxes, RigidBody};


pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)));
    }
}

pub fn setup(mut cmds: Commands, _my_assets: Res<MyAssets>) {
    let mut map = HashMap::new();
    map.insert(
        "walk".to_string(),
        _my_assets.slime_animation_walking.clone(),
    );

    cmds.spawn((
        Mob,
        Boss,
        Stats {
            hp: 100,
            max_hp: 100,
            attack: 20,
            speed: 5.0,
            score: 100,
        },
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(10.0, 400.0, 2.0).looking_to(Vec3::Z, Vec3::Y).with_scale((5.0,5.0,5.0).into()),
            ..default()
        },
        Collider::cuboid(5.0, 5.0, 5.0),
    ))
    .with_children(|mob| {
        mob.spawn(SceneBundle {
            scene: _my_assets.slime.clone(),
            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
            ..default()
        });
    })
    .insert(AnimationController { done: false })
    .insert(Animations(map))
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(ActiveEvents::COLLISION_EVENTS);
}

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Boss;
