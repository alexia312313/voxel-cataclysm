use crate::{
    voxel::{mob::Mob, player::Player, Attacked, Stats},
    GameState,
};
use bevy::{prelude::*, transform, window::PrimaryWindow};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};

// system that listen if an entity is attacked
pub fn entity_attacked_handler(
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

fn player_melee_attack(
    mut commands: Commands,
    transform_query: Query<&Transform>,
    stats_query: Query<&Stats>,
    player_query: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    button: Res<Input<MouseButton>>,
) {
    if button.just_pressed(MouseButton::Left) {
        let player_entity = player_query.single();
        let player_transform = transform_query.get(player_entity).unwrap();
        // We will color in read the colliders hovered by the mouse.
        for (camera, camera_transform) in &camera_query {
            // First, compute a ray from the mouse position.
            let Some(ray) = camera.viewport_to_world(camera_transform, Vec2::new(439.4032, 616.9291)) else { return; };

            // Then cast the ray.
            let hit = rapier_context.cast_ray(
                ray.origin,
                ray.direction,
                f32::MAX,
                true,
                QueryFilter::only_dynamic(),
            );

            if let Some((entity, _toi)) = hit {
                let mob_transform = transform_query.get(entity).unwrap();
                let mob_stats = stats_query.get(entity).unwrap();
                if player_transform
                    .translation
                    .distance(mob_transform.translation)
                    > 5.0
                {
                    commands.entity(entity).insert(Attacked {
                        damage: mob_stats.attack,
                    });
                    print!("{:?}", entity)
                }
            }
        }
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
        app.add_systems(
            (
                entity_attacked_handler,
                despawn_dead_mobs,
                player_melee_attack,
            )
                .in_set(OnUpdate(GameState::Game)),
        );
    }
}
