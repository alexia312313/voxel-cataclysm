use super::Stats;
use crate::{
    voxel::{
        animation::{AnimationController, Animations},
        loading::MyAssets,
    },
    GameState,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, GravityScale, LockedAxes, RigidBody};

pub mod brain;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(brain::BrainHandlerPlugin);
    }
}

#[derive(Component, Debug, Clone)]
pub struct Mob(pub String);

