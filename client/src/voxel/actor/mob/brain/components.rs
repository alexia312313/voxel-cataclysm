// First, we define a "Thirst" component and associated system. This is NOT
// THE AI. It's a plain old system that just makes an entity "thirstier" over
// time. This is what the AI will later interact with.
//
// There's nothing special here. It's a plain old Bevy component.
use bevy::prelude::*;

use crate::voxel::networking::ControlledPlayer;

#[derive(Component, Debug)]
pub struct Aggro {
    pub per_second: f32,
    pub aggro: f32,
    pub target: Entity,
}

impl Aggro {
    pub const fn new(aggro: f32, per_second: f32, target: Entity) -> Self {
        Self {
            aggro,
            per_second,
            target,
        }
    }
}

pub fn aggro_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Aggro)>,
    player_query: Query<(&Transform, Entity), With<ControlledPlayer>>,
) {
    for (mob_pos, mut aggro) in query.iter_mut() {
        let mut closest_player = std::f32::MAX;
        for (player_pos, entity) in player_query.iter() {
            let distance = mob_pos.translation.distance(player_pos.translation);
            if closest_player > distance {
                closest_player = distance;
                aggro.target = entity;
            }
            if distance < 100.0 {
                aggro.aggro += aggro.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                if aggro.aggro >= 100.0 {
                    aggro.aggro = 100.0;
                }
            }
            trace!(
                "Aggro: {}, Targeted Player: {:?}",
                aggro.aggro,
                aggro.target
            );
        }
    }
}
