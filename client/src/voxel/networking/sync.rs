use super::{ClientLobby, NetworkMapping};
use crate::{
    voxel::{
        animation::Animations,
        loading::MyAssets,
        networking::{ControlledPlayer, PlayerInfo},
        player::{
            bundle::{BasePlayerBundle, MyCamera3dBundle, PlayerColliderBundle, PlayerHeadBundle},
            Body,
        },
    },
    GameState,
};
use bevy::{
    prelude::{shape::Icosphere, *},
    utils::HashMap,
};
use bevy_renet::renet::{transport::NetcodeClientTransport, RenetClient};
use common::{
    ClientChannel, NetworkedEntities, PlayerCommand, PlayerInput, ServerChannel, ServerMessages,
};

fn sync_players(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    transport: Res<NetcodeClientTransport>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
    _my_assets: Res<MyAssets>,
) {
    let client_id = transport.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate {
                id,
                translation,
                entity,
            } => {
                println!("Player {} connected.", id);

                let mut map = HashMap::new();
                map.insert("walk".to_string(), _my_assets.player_animation_walk.clone());
                map.insert("hit".to_string(), _my_assets.player_animation_hit.clone());

                let mut client_entity = cmds.spawn((
                    BasePlayerBundle::default(),
                    Animations(map),
                    TransformBundle {
                        local: Transform::from_xyz(translation[0], translation[1], translation[2])
                            .looking_to(Vec3::Z, Vec3::Y),
                        ..default()
                    },
                ));

                if client_id == id {
                    client_entity
                        .insert(ControlledPlayer)
                        .with_children(|player| {
                            player.spawn(Body).insert(SceneBundle {
                                scene: _my_assets.player.clone(),
                                transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
                                ..default()
                            });
                            player.spawn(PlayerColliderBundle::default());
                            player
                                .spawn(PlayerHeadBundle::default())
                                .with_children(|head| {
                                    head.spawn(MyCamera3dBundle::default());
                                });
                        });
                } else {
                    client_entity.with_children(|player| {
                        player.spawn(SceneBundle {
                            scene: _my_assets.player.clone(),
                            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
                            ..default()
                        });
                    });
                }

                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: client_entity.id(),
                };
                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some(PlayerInfo {
                    server_entity,
                    client_entity,
                }) = lobby.players.remove(&id)
                {
                    cmds.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
            ServerMessages::SpawnProjectile {
                entity,
                translation,
            } => {
                let projectile_entity = cmds.spawn(PbrBundle {
                    mesh: meshes.add(
                        Mesh::try_from(Icosphere {
                            radius: 0.1,
                            subdivisions: 5,
                        })
                        .unwrap(),
                    ),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_translation(translation.into()),
                    ..Default::default()
                });
                network_mapping.0.insert(entity, projectile_entity.id());
            }
            ServerMessages::DespawnProjectile { entity } => {
                if let Some(entity) = network_mapping.0.remove(&entity) {
                    cmds.entity(entity).despawn();
                }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translations[i].into();
                let rotation = networked_entities.rotations[i].into();
                let transform = Transform {
                    rotation,
                    translation,
                    ..Default::default()
                };
                cmds.entity(*entity).insert(transform);
            }
        }
    }
}

fn sync_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message);
}

fn sync_rotation(body_rot: Query<&Transform, With<Body>>, mut client: ResMut<RenetClient>) {
    if let Err(_) = body_rot.get_single() {
        return;
    }
    let rotation = body_rot.single();

    let message = bincode::serialize(&rotation.rotation).unwrap();
    client.send_message(ClientChannel::Rots, message)
}

fn sync_player_commands(
    mut player_commands: EventReader<PlayerCommand>,
    mut client: ResMut<RenetClient>,
) {
    for command in player_commands.iter() {
        let command_message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::Command, command_message);
    }
}

pub struct NetSyncPlugin;
impl Plugin for NetSyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            (
                sync_rotation,
                sync_input,
                sync_player_commands,
                sync_players,
            )
                .distributive_run_if(bevy_renet::transport::client_connected)
                .in_set(OnUpdate(GameState::Game)),
        );
    }
}
