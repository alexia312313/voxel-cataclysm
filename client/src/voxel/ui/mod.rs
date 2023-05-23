use crate::GameState;
use bevy::prelude::*;
use update::*;

mod build;
pub mod dead;
pub mod end;
mod spawn;
mod styles;
mod update;
pub(crate) mod chat;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            //plugins
            .add_plugin(spawn::SpawnHandlerPlugin)
            //systems
            .add_systems((update_health_text, update_score_text).in_set(OnUpdate(GameState::Game)));
    }
}

#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct SuperUIs;

#[derive(Component)]
pub struct UiHealth;

#[derive(Component)]
pub struct UiCrosshair;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;
