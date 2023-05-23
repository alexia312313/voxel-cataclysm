use bevy::prelude::*;

pub mod bundle;
pub mod controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(controller::PlayerControllerPlugin);
    }
}

/// Marker component for player body.
#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct MobSpawnTimer {
    pub get_timer: Timer,
    pub current_mobs: u32,
    pub max_mobs: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum CameraMode {
    FirstPerson,
    ThirdPersonForward,
}

impl CameraMode {
    const fn next(self) -> Self {
        match self {
            Self::FirstPerson => Self::ThirdPersonForward,
            Self::ThirdPersonForward => Self::FirstPerson,
        }
    }
    fn translation(self) -> Vec3 {
        match self {
            Self::FirstPerson => Vec3::ZERO,
            Self::ThirdPersonForward => Vec3::Z * -5.0,
        }
    }
}
