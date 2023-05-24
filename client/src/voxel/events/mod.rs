use crate::GameState;
use bevy::prelude::*;

use self::end::{ spawn_arrow, spawn_end_portal, detect_player_v2};

mod end;

pub struct EventsHandlerPlugin;

impl Plugin for EventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_end_portal.in_schedule(OnEnter(GameState::Game)))
            .add_systems((detect_player_v2, spawn_arrow).in_set(OnUpdate(GameState::Game)));
    }
}

#[derive(Component)]
pub struct EndPortal {}


#[derive(Component)]
pub struct EndPortalCollider {}


#[derive(Component)]
pub struct Arrow {
    timer: Timer,
}
