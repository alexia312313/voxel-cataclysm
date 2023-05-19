use crate::voxel::ui::{
    styles::{get_text_style, HEALTH_STYLE, SCORE_STYLE, UI},
    HealthText, ScoreText, UiHealth,
};
use bevy::prelude::*;

use super::{
    styles::{SUPER_UI}, EndScreenCamera2d,
};

pub fn build_end_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let end_screen_entity = commands
        .spawn((
            NodeBundle {
                style: SUPER_UI,
                ..Default::default()
            },
            EndScreenCamera2d,
        ))
                .with_children(|parent| {
                    // Crosshair
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new("+", get_text_style(&asset_server))],
                            alignment: TextAlignment::Center,

                            ..default()
                        },
                        ..default()
                    });
                }) ;
                end_screen_entity
}
