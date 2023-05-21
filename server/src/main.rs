use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::{
    connection_config, ClientChannel, NetworkedEntities, Player, PlayerInput, Projectile,
    RotationInput, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
}

#[derive(Debug, Resource)]
struct BotId(u64);

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    (server, transport)
}

fn main() {
    let mut app = App::new();
    let (server, transport) = new_renet_server();
    app.add_plugin(AssetPlugin::default())
        .add_asset::<Mesh>()
        .add_asset::<Scene>()
        .insert_resource(SceneSpawner::default())
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin)
        .add_plugin(NetcodeServerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ServerLobby::default())
        .insert_resource(BotId(0))
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems((
            server_update_system,
            server_network_sync,
            move_players_system,
        ))
        .run();
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut players: Query<(Entity, &Player, &mut Transform)>,
) {
    for event in server_events.iter() {
        //TODO: ADAPT
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                // Initialize other players for this new client
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                // Spawn new player
                let transform = Transform::from_xyz(
                    (fastrand::f32() - 0.5) * 40.,
                    171.0,
                    (fastrand::f32() - 0.5) * 40.,
                );
                let player_entity = commands
                    .spawn(PbrBundle {
                        transform,
                        ..Default::default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
                    .insert(Collider::capsule_y(0.5, 0.5))
                    .insert(PlayerInput::default())
                    .insert(Player { id: *client_id })
                    .id();

                lobby.players.insert(*client_id, player_entity);

                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *client_id,
                    entity: player_entity,
                    translation,
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }

    for client_id in server.disconnections_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(input);
            }
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Rots) {
            let rots: RotationInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                if let Ok((_, _, mut player_transform)) = players.get_mut(*player_entity) {
                    player_transform.rotation = rots.rotation;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), Or<(With<Player>, With<Projectile>)>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in query.iter() {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation.into());
        networked_entities.rotations.push(transform.rotation);
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

fn move_players_system(mut query: Query<(&mut Transform, &PlayerInput)>) {
    for (mut transform, input) in query.iter_mut() {
        let mut acceleration = 1.0f32;
        let mut direction = Vec3::ZERO;

        let (forward, right) = {
            let forward = transform.rotation.mul_vec3(Vec3::Z).normalize();
            let right = Vec3::Y.cross(forward); // @todo(meyerzinn): not sure why this is the correct orientation
            (forward, right)
        };

        if input.up {
            direction.z -= 1.0;
        }

        if input.down {
            direction.z += 1.0;
        }

        if input.right {
            direction.x += 1.0;
        }

        if input.left {
            direction.x -= 1.0;
        }

        if input.jump {
            direction.y += 1.0;
        }
        if input.crouch {
            direction.y -= 1.0;
        }

        if input.run {
            acceleration *= 8.0;
        }

        if direction == Vec3::ZERO {
            return;
        }

        // hardcoding 0.01 as a factor for now to not go zoomin across the world.
        transform.translation += direction.x * right * acceleration * 0.01
            + direction.z * forward * acceleration * 0.01
            + direction.y * Vec3::Y * acceleration * 0.01;
    }
}
