use crate::GameState;
use bevy::{
    app::AppExit,
    prelude::{shape::Icosphere, *},
    utils::HashMap,
    window::exit_on_all_closed,
};
use bevy_renet::renet::{
    transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
    RenetClient,
};
use common::{connection_config, PlayerCommand, PlayerInput, PROTOCOL_ID};
use std::{net::UdpSocket, time::SystemTime};

mod sync;
mod update;

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (client, transport) = new_renet_client();
        app.add_event::<PlayerCommand>()
            .insert_resource(ClientLobby::default())
            .insert_resource(PlayerInput::default())
            .insert_resource(client)
            .insert_resource(transport)
            .insert_resource(NetworkMapping::default())
            .add_plugin(sync::NetSyncPlugin)
            .add_plugin(update::NetUpdatePlugin)
            .add_system(
                disconnect_on_exit
                    .in_base_set(CoreSet::PostUpdate)
                    .after(exit_on_all_closed),
            )
            .add_system(setup_target.in_schedule(OnEnter(GameState::Game)))
            .add_system(panic_on_error_system.in_set(OnUpdate(GameState::Game)));
    }
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(connection_config());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

fn setup_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(Icosphere {
                    radius: 0.1,
                    subdivisions: 5,
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            ..Default::default()
        })
        .insert(Target);
}

fn disconnect_on_exit(exit: EventReader<AppExit>, mut client: ResMut<RenetClient>) {
    if !exit.is_empty() && client.is_connected() {
        client.disconnect();
    }
}

// If any error is found we just panic
fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<u64, PlayerInfo>,
}

#[derive(Component)]
struct Target;
