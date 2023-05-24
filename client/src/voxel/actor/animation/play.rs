use std::time::Duration;

use super::{link::AnimationEntityLink, AnimationController, Animations};
use crate::GameState;
use bevy::prelude::*;

// TODO: Separate into multiples systems for each animation type and cleaner code
pub fn walk_animation(
    mut player_query: Query<&mut AnimationPlayer>,
    mut query: Query<(
        Changed<Transform>,
        &mut AnimationController,
        &Animations,
        &AnimationEntityLink,
    )>,
    mouse: Res<Input<MouseButton>>,
    mut hit_done: Local<bool>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    for (is_moving, mut controller, animations, player_entity) in query.iter_mut() {
        let mut player = player_query.get_mut(player_entity.0).unwrap();
        let animations = &animations.0;
        let walk_done = &mut controller.done;
        if !*hit_done {
            if is_moving {
                if !*walk_done {
                    if let Some(walk) = animations.get("walk") {
                        player.play(walk.clone()).repeat();
                    }
                    *walk_done = true;
                } else {
                    player.stop_repeating();
                    *walk_done = false;
                }
            }
            if mouse.just_pressed(MouseButton::Left) {
                timer.reset();
                timer.set_duration(Duration::from_secs_f32(1.0));
                if let Some(attack) = animations.get("hit") {
                    player
                        .play(animations.get("walk").unwrap().clone())
                        .set_speed(0.0);
                    player.play(attack.clone()).set_speed(3.0);
                    *hit_done = true;
                }
            }
        } else {
            timer.tick(time.delta());
            // if player finished
            if timer.just_finished() {
                *hit_done = false;
            }
        }
    }
}

//  todo: make this a component
/*
pub fn countdown_animation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ControlledPlayer>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.translation.y = (time.seconds_since_startup() * 2.0).sin() * 0.5;
    }
    println!("{}", time.elapsed_seconds());
}
*/

pub struct AnimationsPlayerPlugin;

impl Plugin for AnimationsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(walk_animation.in_set(OnUpdate(GameState::Game)))
            //.add_system(countdown_animation.in_set(OnUpdate(GameState::Game)))
            ;
    }
}
