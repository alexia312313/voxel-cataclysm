use crate::{
    voxel::{mob::Mob, networking::ControlledPlayer, Attacked, Stats},
    GameState,
};
use bevy::{prelude::*, window::PrimaryWindow};
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
    player_query: Query<(Entity, &Stats), With<ControlledPlayer>>,
    rapier_context: Res<RapierContext>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    button: Res<Input<MouseButton>>,
) {
    if let Ok((player_entity, stats)) = player_query.get_single() {
        if button.just_pressed(MouseButton::Left) {
            let player_transform = transform_query.get(player_entity).unwrap();
            let window = windows.single();
            let Some(cursor_position) = window.cursor_position() else { return; };
            // We will color in read the colliders hovered by the mouse.
            for (camera, camera_transform) in &camera_query {
                // First, compute a ray from the mouse position.
                let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };
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

                    if player_transform
                        .translation
                        .distance(mob_transform.translation)
                        > 5.0
                    {
                        commands.entity(entity).insert(Attacked {
                            damage: stats.attack,
                        });
                        print!("{:?}", entity)
                    }
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
