use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{Collider, GravityScale, RigidBody, LockedAxes, ActiveEvents};
use rand::prelude::*;
use crate::{GameState, voxel::loading::MyAssets};

use super::{mob::Mob, Stats, animation::{AnimationController, Animations}};
pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)));
    }
}

pub fn setup(mut cmds: Commands, _my_assets: Res<MyAssets>) {

    let rndm = (random::<f32>*100.0,200.0,random::<f32>*100)
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
            attack: 10,
            speed: 5.0,
            score: 10,
        },
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(rndm.0,rndm.1,rndm.2)
                .looking_to(Vec3::Z, Vec3::Y)
                .with_scale((10.0, 10.0, 10.0).into()),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        GravityScale(0.0),
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
pub struct Boss;
