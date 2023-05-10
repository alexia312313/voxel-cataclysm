use bevy::prelude::*;

use crate::GameState;

use super::loading::MyAssets;

mod animation_link;
mod animation_play;

pub struct AnimationsHandlerPlugin;

impl Plugin for AnimationsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)))
            .add_plugin(animation_link::AnimationLinkingPlugin)
            .add_plugin(animation_play::AnimationsPlayerPlugin);
    }
}

// Inserting animation clips into resources
pub fn setup(mut cmds: Commands, _my_assets: Res<MyAssets>) {
    cmds.insert_resource(WalkAnimations(vec![
        _my_assets.slime_animation_walking.clone(),
        _my_assets.player_animation_walking.clone(),
    ]));
    cmds.insert_resource(HitAnimations(vec![_my_assets.player_animation_hit.clone()]));
    cmds.insert_resource(IdleAnimations(vec![_my_assets
        .player_animation_idle
        .clone()]));
}

#[derive(Resource)]
pub struct WalkAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct HitAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct IdleAnimations(pub Vec<Handle<AnimationClip>>);
