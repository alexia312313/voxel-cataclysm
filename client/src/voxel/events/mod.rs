use crate::GameState;
use bevy::prelude::*;

use self::end::{detect_player, spawn_end_portal, find_portal_position};

mod end;

pub struct EventsHandlerPlugin;

impl Plugin for EventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_end_portal.in_schedule(OnEnter(GameState::Game)))
            .add_systems((
                detect_player,
                find_portal_position
            ).in_set(OnUpdate(GameState::Game)));
    }
}

#[derive(Component)]
pub struct EndPortal {}
