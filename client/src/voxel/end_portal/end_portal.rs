use bevy::prelude::*;

use crate::voxel::{end_portal::EndPortal, loading::MyAssets};

pub fn spawn_end_portal(
   mut commands: Commands, 
    _my_assets: Res<MyAssets>)
    { 
        println!("portal loaded");
        commands.spawn((
        SceneBundle{
            scene: _my_assets.end_portal.clone_weak(),
            transform:Transform::from_xyz(0.0, 200.0, 0.0),
            ..Default::default()
        },
        EndPortal{}
    ));

    
}       

pub fn despawn_end_portal(    
    mut commands: Commands,
    portal_query:Query<Entity, With<EndPortal>>
){
    if let Ok (portal_entity)= portal_query.get_single(){
        commands.entity(portal_entity).despawn_recursive();
    }
}