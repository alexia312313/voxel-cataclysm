use crate::voxel::ui::{styles::{get_text_style,get_text_style_title }, end::{styles::{SUPER_UI,GAME_OVER_STYLE, SCORE_BOX_STYLE, TIME_BOX_STYLE, QUIT_BUTTON_STYLE}, FinalScoreText, FinalTime, QuitButton}, dead::DeadScreenCamera2d};

use bevy::{prelude::*, window::PrimaryWindow};

use super::DeadScreenUI;



pub fn spawn_dead_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {

    println!("dead screen");
    build_dead_screen(&mut commands, &asset_server);
    //build_ui_crosshair(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    println!("spawn end screen");
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
                                get_text_style_title(&asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });
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
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText,
                    ));
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
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalTime,
                    ));
                });


             
                
                parent
                .spawn((ButtonBundle {
                    style: QUIT_BUTTON_STYLE,
                    background_color:BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                QuitButton{},
                )
            ).with_children(|parent|{
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "QUIT",
                            get_text_style(&asset_server),
                        )],
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

