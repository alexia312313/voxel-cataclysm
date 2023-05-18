use crate::voxel::ui::styles::get_text_style;
use bevy::prelude::*;

use super::styles::CROSSHAIR_STYLE;

pub fn build_ui_crosshair(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let ui_crosshair_entity = commands
        .spawn(NodeBundle {
            style: CROSSHAIR_STYLE,
            ..default()
        })
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
        })
        .id();
    ui_crosshair_entity
}
