use crate::{
    voxel::{mob::Mob, Attacked, Stats},
    GameState,
};
use bevy::prelude::*;

// system that listen if an entity is attacked
pub fn attack_handler(
    mut cmds: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Stats, &Attacked)>,
) {
    for (entity, mut transform, mut stats, attacked) in query.iter_mut() {
        // move back
        let move_back = transform.back();
        transform.translation += move_back * time.delta_seconds() * 100.0;
        // apply damage
        stats.hp -= attacked.damage;
        // reset attacked component
        cmds.entity(entity).remove::<Attacked>();
    }
}

// system that despawn dead mobs
pub fn despawn_dead_mobs(mut cmds: Commands, mut query: Query<(Entity, &Stats), With<Mob>>) {
    for (entity, stats) in query.iter_mut() {
        if stats.hp <= 0 {
            cmds.entity(entity).despawn_recursive();
        }
    }
}

pub struct EventHandlerPlugin;
impl Plugin for EventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems((attack_handler, despawn_dead_mobs).in_set(OnUpdate(GameState::Game)));
    }
}
