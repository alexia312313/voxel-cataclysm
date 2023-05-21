use bevy::prelude::*;

use crate::{
    voxel::{networking::ControlledPlayer, Stats},
    GameState,
};

use self::{
    end::spawn_end_screen,
    updates::{interact_with_quit_button, update_score_text_final, update_time_final},
};

pub mod end;
pub mod styles;
pub mod updates;

#[derive(Component)]
pub struct EndScreenCamera2d;


#[derive(Component,Debug)]
pub struct ElapsedTime{
    pub elapsed:f32
}

#[derive(Component)]
pub struct EndScreenUI;

#[derive(Component)]
pub struct FinalScoreText;

#[derive(Component)]
pub struct FinalTime;

#[derive(Component)]
pub struct QuitButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_end_screen.in_schedule(OnEnter(GameState::GameOver)))
            .add_systems(
                (
                    update_score_text_final,
                    interact_with_quit_button,
                    update_time_final, 
                )
                    .in_set(OnUpdate(GameState::GameOver)))
            .add_system(add_score.in_set(OnUpdate(GameState::Game)))
            .add_system(despawn_game_over.in_schedule(OnExit(GameState::GameOver)));
    }
}

pub fn despawn_game_over(
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

//testing
fn add_score(mut player_q: Query<&mut Stats, With<ControlledPlayer>>) {
    for mut player in player_q.iter_mut() {
        player.score += 1;
    }
}
