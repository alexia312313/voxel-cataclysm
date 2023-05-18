use bevy::prelude::*;

pub mod animation;
pub mod mob;
pub mod player;

pub struct ActorPlugin;
impl Plugin for ActorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(player::PlayerPlugin)
            .add_plugin(mob::MobPlugin)
            .add_plugin(animation::AnimationsHandlerPlugin);
    }
}

#[derive(Component)]
pub struct Attacked {
    pub damage: i32,
}

#[derive(Component)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub speed: f32,
    pub score: i32,
}
