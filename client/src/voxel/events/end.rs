use std::ops::{AddAssign, Div, Sub};

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierContext, Sensor};

use crate::{
    voxel::{
        events::EndPortal, loading::MyAssets, networking::ControlledPlayer, CurrentLocalPlayerChunk,
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

pub fn find_portal_position(
    portal_q: Query<&Transform, With<EndPortal>>,
    player_pos: Query<&Transform, (With<ControlledPlayer>, Without<EndPortal>)>,
) {
    for portal in portal_q.iter() {
        for pos in player_pos.iter() {
            let portal_pos = portal.translation;
            let player_pos = pos.translation;
            let mag = player_pos.distance(portal_pos);

            let vec_comps = portal_pos - player_pos;
            let dic = vec_comps.div(mag);

            //  println!("portal position{}", portal_pos);
            //  println!("player position {}", player_pos);
            //  println!("mag {}", mag);
            //println!("dic {}", dic);
        }
    }
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
    player_q: Query<&Transform, With<ControlledPlayer>>,
    keyboard_input: Res<Input<KeyCode>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::P) {
        
        for player in player_q.iter(){

            let translation = player.translation;
            let above = Vec3::new(0.0,3.0,0.0);
            let combined = translation + above;
            commands.spawn((
                SceneBundle {
                    scene: _my_assets.arrow.clone_weak(),
                    transform: Transform::from_translation(combined),
                    ..Default::default()
                },
                Arrow {},
            ));


            println!("spawns arrow")
        }

    }
  
}

