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
    connection_config, ClientChannel, Mob, MobInfo, NetworkedEntities, NonNetworkedEntities,
    Player, PlayerInput, RotationInput, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use std::{collections::HashMap, f32::consts::PI, net::UdpSocket, time::SystemTime};

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
            //server_non_network_sync,
        ))
        .run();
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut players: Query<(Entity, &Player)>,
    mut mobs: Query<(&mut Stats, &Mob, Entity)>,
    mut transform: Query<&mut Transform>,
) {
    for event in server_events.iter() {
        //TODO: ADAPT
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                if lobby.players.is_empty() {
                    let host = true;
                    let message = bincode::serialize(&host).unwrap();
                    server.send_message(*client_id, ServerChannel::Host, message);
                }
                for (entity, player) in players.iter() {
                    let transform = transform.get_mut(entity).unwrap();
                    let translation: [f32; 3] = transform.translation.into();
                    let message: Vec<u8> = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }
                let transform = Transform::from_xyz(
                    (fastrand::f32() - 0.5) * 40.,
                    171.0,
                    (fastrand::f32() - 0.5) * 40.,
                )
                .with_rotation(Quat::from_rotation_y(PI));
                let player_entity = commands
                    .spawn(PbrBundle {
                        transform,
                        ..Default::default()
                    })
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
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                if let Ok((e, _)) = players.get_mut(*player_entity) {
                    let mut player_transform = transform.get_mut(e).unwrap();

                    player_transform.translation = input.translation;
                }
            }
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Rots) {
            let rots: RotationInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                if let Ok((e, _)) = players.get_mut(*player_entity) {
                    let mut player_transform = transform.get_mut(e).unwrap();

                    player_transform.rotation = rots.rotation;
                }
            }
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::Mobs) {
            let mob: MobInfo = bincode::deserialize(&message).unwrap();
            for (mut stat, id, entity) in mobs.iter_mut() {
                let mut transform = transform.get_mut(entity).unwrap().clone();
                if mob.mob_id == id.0 {
                    if mob.mob_id == id.0 {
                        transform.translation = mob.translation;
                        transform.rotation = mob.rotation;
                        stat.0 = mob.hp;
                    }
                    let message = bincode::serialize(&entity).unwrap();
                    server.broadcast_message(ServerChannel::NonNetworkedEntities, message);
                }
            }
        }
        while let Some(message) = server.receive_message(client_id, ClientChannel::NewMob) {
            let mob: Entity = bincode::deserialize(&message).unwrap();
            println!("New mob: {:?}", mob);
        }
    }
}

#[derive(Component)]
pub struct Stats(i32);

#[allow(clippy::type_complexity)]
fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<Player>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in query.iter() {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation.into()); //Vec3
        networked_entities.rotations.push(transform.rotation); //Quat
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

/*fn server_non_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform), With<Mob>>,
) {
    let mut non_networked_entities = NonNetworkedEntities::default();
    for (entity, transform) in query.iter() {
        non_networked_entities.entities.push(entity);
        non_networked_entities
            .translations
            .push(transform.translation.into()); //Vec3
        non_networked_entities.rotations.push(transform.rotation); //Quat
    }
    let sync_message = bincode::serialize(&non_networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NonNetworkedEntities, sync_message);
} */
