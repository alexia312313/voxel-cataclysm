use crate::voxel::ui::{
    dead::DeadScreenCamera2d,
    end::{
        styles::{GAME_OVER_STYLE, QUIT_BUTTON_STYLE, SCORE_BOX_STYLE, SUPER_UI, TIME_BOX_STYLE},
        ElapsedTime, FinalScoreText, FinalTime, QuitButton,
    },
    styles::{get_text_style, get_text_style_title},
};

use bevy::{prelude::*, window::PrimaryWindow};

use super::DeadScreenUI;

pub fn spawn_dead_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    let sound = asset_server.load("audio/pain.ogg");
    audio.play(sound);
    let elapsed_t = time.elapsed_seconds_wrapped();
    commands.spawn(ElapsedTime { elapsed: elapsed_t });
    build_dead_screen(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            camera: Camera {
                order: (1),
                ..default()
            },
            ..default()
        },
        DeadScreenCamera2d {},
    ));
}

pub fn build_dead_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let end_screen_entity = commands
        .spawn((
            NodeBundle {
                style: SUPER_UI,
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            DeadScreenUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: GAME_OVER_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "You died",
                                get_text_style_title(asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: TIME_BOX_STYLE,
                    //      background_color:BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Time: ",
                                get_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(asset_server))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalTime,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: SCORE_BOX_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Score: ",
                                get_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(asset_server))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText,
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: QUIT_BUTTON_STYLE,
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("QUIT", get_text_style(asset_server))],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    end_screen_entity
}
