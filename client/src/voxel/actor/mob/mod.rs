use bevy::prelude::*;

pub mod brain;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(brain::BrainHandlerPlugin);
    }
}
