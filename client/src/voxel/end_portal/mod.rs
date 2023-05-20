use crate::GameState;
use bevy::{prelude::*, ui::update};

use self::end_portal::{spawn_end_portal, despawn_end_portal, detect_player};

mod end_portal;

pub struct EndPlugin;

impl Plugin for EndPlugin{
    fn build (&self,app:&mut App){
        app.add_system(spawn_end_portal.in_schedule(OnEnter(GameState::Game)))

        .add_system((detect_player).in_set(OnUpdate(GameState::Game)))
        .add_systems((
            despawn_end_portal,
        ).in_schedule(OnExit(GameState::Game))); 
        
    }
}

#[derive(Component)]

pub struct EndPortal{}