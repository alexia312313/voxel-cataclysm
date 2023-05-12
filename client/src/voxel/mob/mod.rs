use crate::GameState;

use self::brain::BrainHandlerPlugin;
use bevy::{prelude::*, utils::HashMap};

use super::loading::MyAssets;

pub mod brain;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Game)))
            .add_plugin(BrainHandlerPlugin);
    }
}

pub fn setup(mut cmds: Commands, _my_assets: Res<MyAssets>) {
    let mut map = HashMap::new();
    map.insert(
        "walk".to_string(),
        _my_assets.slime_animation_walking.clone(),
    );

    cmds.spawn((
        Mob,
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(2.0, 170.0, 2.0).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
    ))
    .with_children(|mob| {
        mob.spawn(Body).insert(SceneBundle {
            scene: _my_assets.slime.clone(),
            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
            ..default()
        });
    });
}

#[derive(Component)]
pub struct Mob;

/// Marker component for player body.
#[derive(Component)]
pub struct Body;
