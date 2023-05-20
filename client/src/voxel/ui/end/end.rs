use crate::voxel::ui::{
    styles::{get_text_style},
};

use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    styles::{SUPER_UI}, EndScreenCamera2d, EndScreenUI,
};

pub fn build_end_screen(
    commands: &mut Commands, asset_server: &Res<AssetServer>
) -> Entity {
    let end_screen_entity = commands
        .spawn((
            NodeBundle {
                style: SUPER_UI,
                ..Default::default()
            },
            EndScreenUI,
        ))
                .with_children(|parent| {

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("End Screen ", get_text_style(&asset_server))],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });
                }).id() ;
                end_screen_entity
}

pub fn spawn_end_screen(
   mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_end_screen(&mut commands, &asset_server);
    //build_ui_crosshair(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    println!("spawn end screen");
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                camera: Camera {
                    order: (1),
                    ..default()
                },
                ..default()
            },
            EndScreenCamera2d {},
        ));}
