use bevy::prelude::*;

pub mod events;
pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(events::EventHandlerPlugin);
    }
}

#[derive(Component)]
pub struct Attacked {
    pub damage: i32,
}
