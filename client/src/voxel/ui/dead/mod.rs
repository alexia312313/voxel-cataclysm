use bevy::prelude::*;

use crate::GameState;

use self::dead::spawn_dead_screen;

use super::end::updates::{update_score_text_final, interact_with_quit_button};

pub mod dead;
pub struct DeadPlugin;

impl Plugin for DeadPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_dead_screen.in_schedule(OnEnter(GameState::Dead)))
        .add_system(update_score_text_final.in_set(OnUpdate(GameState::Dead)))

        .add_system(interact_with_quit_button.in_set(OnUpdate(GameState::Dead)))

        .add_system(despawn_dead.in_schedule(OnExit(GameState::Dead)));
    }
}


#[derive(Component)]
pub struct DeadScreenUI;


#[derive(Component)]
pub struct DeadScreenCamera2d;


pub fn despawn_dead (
    mut commands: Commands,
    dead_screen_cam_query: Query<Entity, With<DeadScreenCamera2d>>,
    dead_screen_ui_query: Query<Entity, With<DeadScreenUI>>,
) {
    if let Ok(dead_screen_cam_entity) = dead_screen_cam_query.get_single() {
        commands.entity(dead_screen_cam_entity).despawn_recursive();
    }
    if let Ok(dead_screen_ui_entity) = dead_screen_ui_query.get_single() {
        commands.entity(dead_screen_ui_entity).despawn_recursive();
    }


}
