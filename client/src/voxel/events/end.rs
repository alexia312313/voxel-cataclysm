use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierContext, RigidBody, Sensor};

use crate::{
    voxel::{
        events::EndPortal,
        loading::MyAssets,
        networking::{ControlledPlayer, ControlledPlayerCollider},
        Stats,
    },
    GameState,
};

use super::{Arrow, EndPortalCollider};

pub fn spawn_end_portal(mut commands: Commands, _my_assets: Res<MyAssets>) {
    commands
        .spawn((
            SceneBundle {
                scene: _my_assets.end_portal.clone_weak(),
                transform: Transform::from_xyz(0.0, 220.0, 0.0),
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

pub fn detect_player_v2(
    rapier_context: Res<RapierContext>,
    portal_query: Query<Entity, With<EndPortalCollider>>,
    player_q: Query<Entity, (With<ControlledPlayerCollider>, Without<EndPortalCollider>)>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for portal in portal_query.iter() {
        for player in player_q.iter() {
            if rapier_context.intersection_pair(portal, player) == Some(true) {
                println!(
                    "The entities {:?} and {:?} have intersecting colliders!",
                    portal, player
                );
                game_state_next_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn spawn_arrow(
    mut commands: Commands,
    _my_assets: Res<MyAssets>,
    keyboard_input: Res<Input<KeyCode>>,
    portal_q: Query<&Transform, With<EndPortal>>,
    mut player: Query<(&Transform, &mut Stats), (With<ControlledPlayer>, Without<EndPortal>)>,
    mut track_arrow: Query<(Entity, &mut Arrow)>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        for portal in portal_q.iter() {
            for (pos, mut stats) in player.iter_mut() {
                if stats.score > 99 {
                    stats.score -= 100;
                    let portal_pos = portal.translation;
                    let translation = pos.translation;
                    let above = Vec3::new(0.0, 3.0, 0.0);
                    let combined = translation + above;
                    let up = pos.up();
                    commands.spawn((
                        SceneBundle {
                            scene: _my_assets.arrow.clone_weak(),
                            transform: Transform::from_translation(combined)
                                .looking_at(portal_pos, up),
                            ..Default::default()
                        },
                        Arrow {
                            timer: Timer::from_seconds(5.0, TimerMode::Once),
                        },
                    ));
                }
            }
        }
    }

    for (entity, mut arrow_time) in track_arrow.iter_mut() {
        arrow_time.timer.tick(time.delta());

        if arrow_time.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
