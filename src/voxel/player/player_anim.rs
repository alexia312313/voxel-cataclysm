use crate::debug::DebugUISet;
use crate::voxel::PlayerController;
use bevy::prelude::*;
use bevy::utils::HashMap;

use super::PlayerAnimations;

pub fn handle_player_movement(
    query: Query<(&PlayerController, &Transform)>,
    anim_query: Res<PlayerAnimations>,
    mut done: Local<bool>,
) {
    let (controller, transform) = query.single();
    let player = anim_query.1;
    let animations = anim_query.0;

    if controller.prev_xyz != transform.translation && !*done {
        player.play(animations.0[1].clone_weak()).repeat();
        *done = true;
    } else {
        *done = false;
    }
}

pub fn set_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut cmds: Commands,
    asset: Res<AssetServer>,
) {
    let mut index = 0;
    for animation_entity in query.iter_mut() {
        index += 1;
        if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity.0) {
            match index {
                1 => {
                    cmds.insert_resource(PlayerAnimations(
                        vec![
                            // idle animation
                            asset.load("models/player/mereo.glb#Animation0"),
                            // walk animation
                            asset.load("models/player/mereo.glb#Animation1"),
                            // hit animation
                            asset.load("models/player/mereo.glb#Animation2"),
                        ],
                        animation_player,
                    ));
                }
                2 => print!("TODO slime animations"),
            }
        }
    }
}

pub fn handle_player_actions() {}

pub struct PlayerAnimationsHandlePlugin;

impl Plugin for PlayerAnimationsHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (handle_player_movement, handle_player_actions)
                .chain()
                .in_base_set(CoreSet::Update)
                .after(DebugUISet::Display),
        );
    }
}
