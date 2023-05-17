use self::link::link_animations;
use crate::GameState;
use bevy::{prelude::*, utils::HashMap};

mod link;
mod play;

pub struct AnimationsHandlerPlugin;

impl Plugin for AnimationsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(link_animations.in_set(OnUpdate(GameState::Game)))
            .add_plugin(play::AnimationsPlayerPlugin);
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub done: bool,
}

#[derive(Component)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);
