use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierContext, Sensor};

use crate::voxel::Stats;
use crate::voxel::mob::Mob;
use crate::voxel::ui::end::end::*;

use crate::voxel::{
    end_portal::EndPortal, loading::MyAssets, networking::ControlledPlayer,
};

pub fn spawn_end_portal(mut commands: Commands, _my_assets: Res<MyAssets>) {
    println!("portal loaded");
    commands
        .spawn((
            SceneBundle {
                scene: _my_assets.end_portal.clone_weak(),
                transform: Transform::from_xyz(0.0, 200.0, 0.0),
                ..Default::default()
            },
            EndPortal {},
        ))
        .insert(Collider::cuboid(10.0, 10.0, 10.0))
        .insert(Sensor);
}

pub fn despawn_end_portal(mut commands: Commands, portal_query: Query<Entity, With<EndPortal>>) {
    if let Ok(portal_entity) = portal_query.get_single() {
        commands.entity(portal_entity).despawn_recursive();
    }
}

pub fn detect_player(
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<ControlledPlayer>>,
    portal_query: Query<Entity, With<EndPortal>>,
) {

    for portal in portal_query.iter(){
        for player in player_query.iter(){
            if rapier_context.intersection_pair(portal,player) == Some(true){
                println!("The colliders {:?} and {:?} are intersecting!", portal, player);
            }

        }
    }   
}


