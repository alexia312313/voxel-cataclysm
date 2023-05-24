use crate::GameState;
use bevy::prelude::*;


use self::{
    end::{detect_player, detect_player_v2, spawn_arrow, spawn_end_portal},
    spawn::spawn_mobs,
};

mod end;
mod spawn;
use super::{networking::ControlledPlayer, Stats};


pub struct EventsHandlerPlugin;

impl Plugin for EventsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (detect_player_v2, spawn_arrow, add_score, spawn_mobs).in_set(OnUpdate(GameState::Game)),
        );
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

fn add_score(mut player_q: Query<&mut Stats, With<ControlledPlayer>>) {
    for mut player in player_q.iter_mut() {
        player.score += 1;
    }
}
