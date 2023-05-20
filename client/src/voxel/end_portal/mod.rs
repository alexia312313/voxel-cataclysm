use crate::GameState;
use bevy::{prelude::*};

use self::end_portal::{spawn_end_portal, detect_player};

mod end_portal;

pub struct EndPlugin;

impl Plugin for EndPlugin{
    fn build (&self,app:&mut App){
        app.add_system(spawn_end_portal.in_schedule(OnEnter(GameState::Game)))

        .add_system(detect_player.in_set(OnUpdate(GameState::Game)));
     
        
    }
}

#[derive(Component)]

pub struct EndPortal{}