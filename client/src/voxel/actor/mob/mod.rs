use crate::{
    voxel::{
        animation::{AnimationController, Animations},
        loading::MyAssets,
    },
    GameState,
};

use self::brain::BrainHandlerPlugin;
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, GravityScale, RigidBody};

use super::Stats;

pub mod brain;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)))
            .add_plugin(BrainHandlerPlugin);
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
        RigidBody::Dynamic,
        Stats {
            hp: 20,
            max_hp: 20,
            attack: 10,
            speed: 5.0,
        },
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(20.0, 170.0, 2.0).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
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
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS);
}

#[derive(Component)]
pub struct Mob;
