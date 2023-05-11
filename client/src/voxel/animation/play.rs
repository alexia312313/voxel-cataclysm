use super::{link::AnimationEntityLink, AnimationController, Animations};
use crate::GameState;
use bevy::prelude::*;

// TODO: Separate into multiples systems for each animation type and cleaner code
pub fn play_animations(
    mut player_query: Query<&mut AnimationPlayer>,
    mut query: Query<(
        Changed<Transform>,
        &mut AnimationController,
        &Animations,
        &AnimationEntityLink,
    )>,
) {
    for (is_moving, mut controller, animations, player_entity) in query.iter_mut() {
        let mut player = player_query.get_mut(player_entity.0).unwrap();
        let animations = &animations.0;
        let done = &mut controller.done;
        if is_moving {
            if !*done {
                if let Some(walk) = animations.get("walk") {
                    player.play(walk.clone()).repeat();
                }
                *done = true;
            }
        } else if let Some(idle) = animations.get("idle") {
            player.play(idle.clone());
            *done = false;
        }
    }
}

pub struct AnimationsPlayerPlugin;

impl Plugin for AnimationsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_animations.in_set(OnUpdate(GameState::Game)));
    }
}
