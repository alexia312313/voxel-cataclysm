use bevy::prelude::*;
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

pub struct NetUpdatePlugin;
impl Plugin for NetUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(update_player_input.in_set(OnUpdate(GameState::Game)));
    }
}
