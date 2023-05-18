use crate::voxel::ui::{
    styles::{get_text_style, HEALTH_STYLE, SCORE_STYLE, UI},
    HealthText, ScoreText, UiHealth,
};
use bevy::prelude::*;

pub fn build_ui_health(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let ui_health_entity = commands
        .spawn((
            NodeBundle {
                style: UI,
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            UiHealth,
        ))
        //.insert(Name::new("UIhealth setup"))
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
                                    "3",
                                    get_text_style(&asset_server),
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
                        ScoreText,
                    ));
                });
        })
        .id();
    ui_health_entity
}
