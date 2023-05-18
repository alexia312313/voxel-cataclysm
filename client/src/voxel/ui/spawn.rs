use crate::voxel::ui::{UICamera, UiHealth};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::crosshair::build_ui_crosshair;
use super::healthbar::build_ui_health;

pub fn spawn_ui_health(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_ui_health(&mut commands, &asset_server);
    build_ui_crosshair(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                camera: Camera {
                    order: (0),
                    ..default()
                },
                ..default()
            },
            UICamera {},
        ))
        .insert(Name::new("UI camera 2d"));
}

pub fn despawn_ui_health(
    mut commands: Commands,
    ui_health_query: Query<Entity, With<UiHealth>>,
    camera_query: Query<Entity, With<UICamera>>,
) {
    if let Ok(ui_health_entity) = ui_health_query.get_single() {
        commands.entity(ui_health_entity).despawn_recursive();
    }
    if let Ok(ui_camera_entity) = camera_query.get_single() {
        commands.entity(ui_camera_entity).despawn();
    }
}

pub struct SpawnHandlerPlugin;

impl Plugin for SpawnHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui_health.in_schedule(OnEnter(GameState::Game)))
            .add_system(despawn_ui_health.in_schedule(OnExit(GameState::Game)));
    }
}
