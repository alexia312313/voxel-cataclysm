use crate::voxel::ui::{
    styles::{get_text_style, HEALTH_STYLE, SCORE_STYLE, UI},
    HealthText, ScoreText, UiHealth,
};
use bevy::prelude::*;

use super::{
    styles::{CROSSHAIR_STYLE, SUPER_UI},
    SuperUIs,
};

pub fn build_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let ui_health_entity = commands
        .spawn((
            NodeBundle {
                style: SUPER_UI,
                ..Default::default()
            },
            SuperUIs,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: CROSSHAIR_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Crosshair
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("+", get_text_style(asset_server))],
                            alignment: TextAlignment::Center,
                            

                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: UI,
                        ..default()
                    },
                    UiHealth,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: HEALTH_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Health
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Health: ",
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
                                        sections: vec![TextSection::new(
                                            "3",
                                            get_text_style(asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                HealthText,
                            ));
                        });

                    parent
                        .spawn(NodeBundle {
                            style: SCORE_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            // Health
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
                                        sections: vec![TextSection::new(
                                            "0",
                                            get_text_style(asset_server),
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ScoreText,
                            ));
                        });
                });
        })
        .id();
    ui_health_entity
}
