use bevy::prelude::*;

use crate::GameState;

use self::end::spawn_end_screen;

pub mod end;
pub mod styles;

#[derive(Component)]
pub struct EndScreenCamera2d;


#[derive(Component)]
pub struct EndScreenUI;


pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.
        add_system(spawn_end_screen.in_schedule(OnEnter(GameState::GameOver)))
        .add_system(despawn_game_over.in_schedule(OnExit(GameState::GameOver)));
    }
}

pub fn despawn_game_over (
    mut commands: Commands,
    end_screen_cam_query: Query<Entity, With<EndScreenCamera2d>>,
    end_screen_ui_query: Query<Entity, With<EndScreenUI>>,
) {
    if let Ok(end_screen_cam_entity) = end_screen_cam_query.get_single() {
        commands.entity(end_screen_cam_entity).despawn_recursive();
    }
    if let Ok(end_screen_ui_entity) = end_screen_ui_query.get_single() {
        commands.entity(end_screen_ui_entity).despawn_recursive();
    }


}