use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use std::f32::consts::PI;

pub mod player_mov;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app //.add_plugin(player_anim::PlayerAnimationsHandlePlugin)
            .add_plugin(player_mov::PlayerControllerPlugin)
            //.add_plugin(animation_link::AnimationLinkPlugin)
            .add_startup_system(setup);
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        Player,
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(2.0, 170.0, 2.0).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
    ))
    .with_children(|player: &mut ChildBuilder| {
        player.spawn(Body).insert(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.8, 0.5))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
            ..default()
        });

        player
            .spawn((
                Head,
                TransformBundle {
                    // head is 1.8m above feet
                    local: Transform::from_translation(Vec3::new(0.0, 0.9, 0.0))
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
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0))
                        .looking_to(Vec3::Z, Vec3::Y),
                    ..Default::default()
                })
                .insert(CameraMode::ThirdPersonForward);
            });
    })
    .insert(Fxaa::default())
    .insert(bevy_atmosphere::plugin::AtmosphereCamera::default());

    // Spawn the sky light
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, SystemSet)]
/// Systems related to player controls.
pub struct PlayerControllerSet;

#[derive(Component)]
pub struct Player;

/// Marker component for player body.
#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Head;

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
