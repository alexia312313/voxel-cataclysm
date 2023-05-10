use crate::voxel::player::{Player, PlayerAnimations};
use crate::GameState;
use bevy::prelude::*;

use super::animation_link::AnimationEntityLink;

pub fn handle_entity_movement(
    query: Query<(&Player, &AnimationEntityLink)>,
    animations: Res<PlayerAnimations>,
    mut player_query: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if let Ok((controller, entity)) = query.get_single() {
        let mut player = player_query.get_mut(entity.0).unwrap();
        if controller.direction.length() > 0.0 {
            if !*done {
                player.play(animations.0[1].clone_weak()).repeat();
                *done = true;
            }
        } else {
            player.stop_repeating();
            *done = false;
        }
    }
}
// TODO: handle player actions
pub fn handle_entity_actions() {}
pub struct AnimationsHandlerPlugin;

impl Plugin for AnimationsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (handle_entity_movement, handle_entity_actions)
                .chain()
                .in_set(OnUpdate(GameState::Game)),
        );
    }
}
