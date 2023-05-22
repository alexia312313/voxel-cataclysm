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

            // lol
            let cursor_position = (window.width() / 2.0, (window.height() / 100.0 * 56.0)).into();
            // let cursor= window.cursor_position();
            //println!("half windows{}",cursor_position);
            //println!("cursor windows{:?}",cursor);

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
               // println!("hit{:?}", hit);
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
    mut player_stats_query: Query<&mut Stats, (With<ControlledPlayer>, Without<Mob>)>,
) {
    for (entity, mob_stats) in mob_stats_query.iter_mut() {
        let mut counter = 0;
        counter += 1;
        if counter > 500 {
            counter -= 500;
            println!("{}", mob_stats.hp);
        }
        if mob_stats.hp <= 0 {
            let mut player_stats = player_stats_query.single_mut();
            cmds.entity(entity).despawn_recursive();
            player_stats.score += mob_stats.score;
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
