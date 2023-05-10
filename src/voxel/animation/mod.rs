use bevy::{prelude::*, utils::HashMap};

mod animation_link;
mod animation_play;

pub struct AnimationsHandlerPlugin;

impl Plugin for AnimationsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(animation_link::AnimationLinkingPlugin)
            .add_plugin(animation_play::AnimationsPlayerPlugin);
    }
}

#[derive(Component)]
pub struct AnimationController {
    pub done: bool,
}

#[derive(Component)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);
