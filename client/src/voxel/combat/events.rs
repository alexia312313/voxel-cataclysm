use crate::{
    voxel::{
        boss::Boss,
        events::{EndPortal, EndPortalCollider},
        loading::MyAssets,
        mob::Mob, networking::ControlledPlayer, player::MobSpawnTimer, AttackWanted, Attacked,
        networking::ControlledPlayer,
    },
    GameState,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{Collider, QueryFilter, RapierContext, RigidBody, Sensor};

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
    player_query: Query<Entity, With<ControlledPlayer>>,
    rapier_context: Res<RapierContext>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    button: Res<Input<MouseButton>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        if button.just_pressed(MouseButton::Left) {
            let player_transform = transform_query.get(player_entity).unwrap();
            let window = windows.single();
            let position = Vec2::new(
                window.resolution.width() / 2.0,
                (window.resolution.height() / 2.0) * 1.25,
            );
            // We will color in read the colliders hovered by the mouse.
            for (camera, camera_transform) in &camera_query {
                // First, compute a ray from the mouse position.
                let Some(ray) = camera.viewport_to_world(camera_transform, position) else { return; };

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
                        println!("AttackWanted Added");
                        commands.entity(entity).insert(AttackWanted);
                    }
                }
            }
        }
    }
}
// system that despawn dead mobs
pub fn despawn_dead_mobs(
    mut cmds: Commands,
    mut mob_stats_query: Query<(Entity, &Stats), With<Mob>>,
    mut player_stats_query: Query<
        (&mut Stats, &mut MobSpawnTimer),
        (With<ControlledPlayer>, Without<Mob>),
    >,
    boss: Query<(Entity, &Transform), With<Boss>>,
    my_assets: Res<MyAssets>,
) {
    for (entity, mob_stats) in mob_stats_query.iter_mut() {
        if mob_stats.hp <= 0 {
            let (mut player_stats, mut timer) = player_stats_query.single_mut();
            timer.current_mobs -= 1;
            cmds.entity(entity).despawn_recursive();
            player_stats.score += mob_stats.score;
            for (boss, tranform) in boss.iter() {
                let pos = tranform.translation;

                if entity == boss {
                    cmds.spawn((
                        SceneBundle {
                            scene: my_assets.end_portal.clone_weak(),
                            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                            ..Default::default()
                        },
                        RigidBody::Fixed,
                        EndPortal {},
                    ))
                    .with_children(|end_portal| {
                        end_portal
                            .spawn(Collider::cuboid(3.22, 3.22, 0.24))
                            .insert(EndPortalCollider {})
                            .insert(Transform::from_xyz(0.0, 3.18, -0.15))
                            .insert(Sensor);
                    });
                }
            }
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
                check_hp,
            )
                .in_set(OnUpdate(GameState::Game)),
        )
        .add_system(end_thing.in_schedule(OnExit(GameState::Game)));
    }
}

pub fn check_hp(
    mut stats: Query<&mut Stats, With<ControlledPlayer>>,
    button: Res<Input<MouseButton>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for mut hp in stats.iter_mut() {
        if hp.hp < 1 {
            game_state_next_state.set(GameState::Dead)
        }
        if button.just_pressed(MouseButton::Right) {
            hp.hp -= 10
        }
    }
}

pub fn end_thing(mut windows: Query<&mut Window>, mut time: ResMut<Time>) {
    for mut window in windows.iter_mut() {
        window.cursor.visible = true;
        time.pause();
    }
}
