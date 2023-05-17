use super::loading::MyAssets;
use crate::{
    voxel::{
        animation::{AnimationController, Animations},
        player::{Body, CameraMode, Head},
        Stats,
    },
    GameState,
};
use bevy::{
    app::AppExit,
    core_pipeline::fxaa::Fxaa,
    ecs::entity,
    input::mouse::MouseMotion,
    prelude::{shape::Icosphere, *},
    utils::HashMap,
    window::{exit_on_all_closed, CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, KinematicCharacterController};
use bevy_renet::renet::{
    transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
    RenetClient,
};
use common::{
    connection_config, ClientChannel, NetworkedEntities, PlayerCommand, PlayerInput, ServerChannel,
    ServerMessages, PROTOCOL_ID,
};
use std::{
    f32::consts::{E, FRAC_PI_2, PI},
    net::UdpSocket,
    thread::spawn,
    time::SystemTime,
};

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
            .add_system(player_input.in_set(OnUpdate(GameState::Game)))
            .add_systems(
                (
                    client_send_input,
                    client_send_player_commands,
                    client_sync_players,
                )
                    .distributive_run_if(bevy_renet::transport::client_connected)
                    .in_set(OnUpdate(GameState::Game)),
            )
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

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
    mouse_button_input: Res<Input<MouseButton>>,
    target_query: Query<&Transform, With<Target>>,
    mut player_commands: EventWriter<PlayerCommand>,
) {
    player_input.run = keyboard_input.pressed(KeyCode::LControl);
    player_input.crouch = keyboard_input.pressed(KeyCode::LShift);
    player_input.left = keyboard_input.pressed(KeyCode::A);
    player_input.right = keyboard_input.pressed(KeyCode::D);
    player_input.up = keyboard_input.pressed(KeyCode::W);
    player_input.down = keyboard_input.pressed(KeyCode::S);
    player_input.jump = keyboard_input.pressed(KeyCode::Space);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let target_transform = target_query.single();
        player_commands.send(PlayerCommand::BasicAttack {
            cast_at: target_transform.translation,
        });
    }
}

fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message);
}

fn client_send_player_commands(
    mut player_commands: EventReader<PlayerCommand>,
    mut client: ResMut<RenetClient>,
) {
    for command in player_commands.iter() {
        let command_message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::Command, command_message);
    }
}

fn client_sync_players(
    mut commands: Commands,
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

                let mut client_entity = commands.spawn((
                    Collider::cuboid(0.4, 0.8, 0.4),
                    Stats {
                        hp: 100,
                        max_hp: 100,
                        attack: 5,
                        speed: 10.0,
                    },
                    VisibilityBundle {
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    TransformBundle {
                        local: Transform::from_xyz(translation[0], translation[1], translation[2])
                            .looking_to(Vec3::Z, Vec3::Y),
                        ..default()
                    },
                ));

                if client_id == id {
                    client_entity
                        .with_children(|player| {
                            player.spawn(Body).insert(SceneBundle {
                                scene: _my_assets.player.clone(),
                                transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
                                ..default()
                            });
                            player
                                .spawn((
                                    Head,
                                    TransformBundle {
                                        // head is 1.8m above feet
                                        local: Transform::from_translation(Vec3::new(
                                            0.0, 0.9, 0.0,
                                        ))
                                        .looking_to(Vec3::Z, Vec3::Y),
                                        ..default()
                                    },
                                ))
                                .with_children(|head| {
                                    // spawn camera as a child of head
                                    head.spawn(Camera3dBundle {
                                        projection: bevy::render::camera::Projection::Perspective(
                                            PerspectiveProjection {
                                                fov: PI / 2.,
                                                far: 2048.0,
                                                ..Default::default()
                                            },
                                        ),
                                        transform: Transform::from_translation(Vec3::new(
                                            0.0, 0.0, -5.0,
                                        ))
                                        .looking_to(Vec3::Z, Vec3::Y),
                                        ..Default::default()
                                    })
                                    .insert(CameraMode::ThirdPersonForward);
                                });
                        })
                        .insert(ControlledPlayer)
                        .insert(Fxaa::default())
                        .insert(Animations(map))
                        .insert(bevy_atmosphere::plugin::AtmosphereCamera::default())
                        .insert(AnimationController { done: false })
                        .insert(KinematicCharacterController::default())
                        .insert(ActiveEvents::COLLISION_EVENTS);
                } else {
                    client_entity
                        .with_children(|player| {
                            player.spawn(SceneBundle {
                                scene: _my_assets.player.clone(),
                                transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
                                ..default()
                            });
                        })
                        .insert(Animations(map))
                        .insert(AnimationController { done: false })
                        .insert(KinematicCharacterController::default())
                        .insert(ActiveEvents::COLLISION_EVENTS);
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
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
            ServerMessages::SpawnProjectile {
                entity,
                translation,
            } => {
                let projectile_entity = commands.spawn(PbrBundle {
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
                    commands.entity(entity).despawn();
                }
            }
            ServerMessages::RotateBody { entity, rotation } => {}
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translations[i].into();
                let transform = Transform {
                    translation,
                    ..Default::default()
                };
                commands.entity(*entity).insert(transform);
            }
        }
    }
}

fn update_target_system(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut target_query: Query<&mut Transform, With<Target>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let mut target_transform = target_query.single_mut();
    if let Some(cursor_pos) = primary_window.single().cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            if let Some(distance) = ray.intersect_plane(Vec3::Y, Vec3::Y) {
                target_transform.translation = ray.direction * distance + ray.origin;
            }
        }
    }
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
