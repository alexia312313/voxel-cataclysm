use crate::GameState;
use bevy::prelude::*;

use super::animation_link::AnimationController;
use super::{HitAnimations, IdleAnimations, WalkAnimations};

// TODO: Separate into multiples systems for each animation type and cleaner code
pub fn play_animations(
    mut player_query: Query<&mut AnimationPlayer>,
    mut controller_query: Query<&mut AnimationController>,
    moving_query: Query<Changed<Transform>>,
    btns: Res<Input<MouseButton>>,
    walk_animations: ResMut<WalkAnimations>,
    idle_animations: ResMut<IdleAnimations>,
    hit_animations: ResMut<HitAnimations>,
) {
    for mut controller in controller_query.iter_mut() {
        let mut player = player_query.get_mut(controller.entity).unwrap();
        if moving_query.get(controller.entity).is_ok() {
            if !controller.done {
                for animation in walk_animations.0.iter() {
                    player.play(animation.clone()).repeat();
                }
                controller.done = true;
            }
        } else {
            for animation in idle_animations.0.iter() {
                player.play(animation.clone()).repeat();
            }
            controller.done = false;
        }
        if btns.just_pressed(MouseButton::Left) {
            for animation in hit_animations.0.iter() {
                player.play(animation.clone());
            }
        }
    }
}

pub struct AnimationsPlayerPlugin;

impl Plugin for AnimationsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_animations.in_set(OnUpdate(GameState::Game)));
    }
}
