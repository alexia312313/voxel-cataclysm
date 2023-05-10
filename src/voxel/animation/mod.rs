use bevy::prelude::*;

mod animation_link;
mod animations;

pub struct AnimationsHandlerPlugin;

impl Plugin for AnimationsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(animation_link::AnimationLinkingPlugin)
            .add_plugin(animations::AnimationsHandlerPlugin);
    }
}
