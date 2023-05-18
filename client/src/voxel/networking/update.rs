use bevy::{prelude::*, window::PrimaryWindow};
use common::{PlayerCommand, PlayerInput};

use crate::GameState;

use super::Target;

fn update_player_input(
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

fn _update_target_system(
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

pub struct NetUpdatePlugin;
impl Plugin for NetUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(update_player_input.in_set(OnUpdate(GameState::Game)));
    }
}
