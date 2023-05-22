use std::ops::{AddAssign, Div, Sub};

use bevy::prelude::*;
use bevy_rapier3d::{
    na::Rotation,
    prelude::{Collider, RapierContext, Sensor},
};

use crate::{
    voxel::{
        events::EndPortal, loading::MyAssets, networking::ControlledPlayer,
        CurrentLocalPlayerChunk, Stats,
    },
    GameState,
};

use super::Arrow;

pub fn spawn_end_portal(mut commands: Commands, _my_assets: Res<MyAssets>) {
    commands.spawn((
        SceneBundle {
            scene: _my_assets.end_portal.clone_weak(),
            transform: Transform::from_xyz(0.0, 150.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(5.0, 5.0, 5.0),
        Sensor,
        EndPortal {},
    ));
}

pub fn detect_player(
    rapier_context: Res<RapierContext>,
    portal_query: Query<Entity, With<EndPortal>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    for portal in portal_query.iter() {
        for (collider1, collider2, intersecting) in rapier_context.intersections_with(portal) {
            if intersecting {
                println!(
                    "The entities {:?} and {:?} have intersecting colliders!",
                    collider1, collider2
                );
                game_state_next_state.set(GameState::GameOver);
            }
        }
    }
}

/*
pub fn detect_player(
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<ControlledPlayer>>,
    portal_query: Query<Entity, With<EndPortal>>,
    mob_query: Query<Entity, With<Mob>>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        for player in player_query.iter() {
            commands.entity(player).despawn_recursive();
            println!("remove controlled player ");
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        for mob in mob_query.iter() {
            commands.entity(mob).despawn_recursive();
            println!("remove mob  ");
        }
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        for portal in portal_query.iter() {
            commands.entity(portal).despawn_recursive();
            println!("remove portal");
        }
    }
    println!(
        "Test1"
    );
    for portal in portal_query.iter() {
        for player in player_query.iter() {
            if rapier_context.intersection_pair(portal, player) == Some(true) {
                println!(
                    "The colliders {:?} and {:?} are intersecting!",
                    portal, player
                );
            }
        }
    }
    println!(
        "Test2"
    );
    for portal in portal_query.iter() {
        for player in player_query.iter() {
            if rapier_context.intersection_pair(player, portal) == Some(true) {
                println!(
                    "The colliders {:?} and {:?} are intersecting!",
                    portal, player
                );
            }
        }
    }
     println!(
        "Test3"
    );
    for portal in portal_query.iter() {
        for player in player_query.iter() {
            for (collider1, collider2, intersecting) in rapier_context.intersections_with(portal) {
                if intersecting {
                    println!("The entities {:?} and {:?} have intersecting colliders!", collider1, collider2);
                }
            }
        }
    }
}
*/

pub fn spawn_arrow(
    mut commands: Commands,
    _my_assets: Res<MyAssets>,
    keyboard_input: Res<Input<KeyCode>>,
    portal_q: Query<&Transform, With<EndPortal>>,
   mut player: Query<(&Transform, &mut Stats), (With<ControlledPlayer>, Without<EndPortal>)>,
) {
    
    if keyboard_input.just_pressed(KeyCode::P) {
        for portal in portal_q.iter() {
            for (pos, mut stats) in player.iter_mut() {
                if stats.score > 99{
                    stats.score -=100;
                    let portal_pos = portal.translation;
                    let translation = pos.translation;
                    let above = Vec3::new(0.0, 3.0, 0.0);
                    let combined = translation + above;
                    let up = pos.up();
                    commands.spawn((
                        SceneBundle {
                            scene: _my_assets.arrow.clone_weak(),
                            transform: Transform::from_translation(combined).looking_at(portal_pos, up),
                            ..Default::default()
                        },
                        Arrow {},
                    ));
                }
               
            }
        }
    }
}


