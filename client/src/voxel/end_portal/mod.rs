use crate::GameState;
use bevy::{prelude::*, ui::update};
use update::*;

use self::end_portal::{spawn_end_portal, despawn_end_portal};

mod end_portal;

pub struct EndPlugin;

impl Plugin for EndPlugin{
    fn build (&self,app:&mut App){
        app.add_system(spawn_end_portal.in_schedule(OnEnter(GameState::Game)))
        .add_system(despawn_end_portal.in_schedule(OnExit(GameState::Game))); 
    }
}

#[derive(Component)]

pub struct EndPortal{}