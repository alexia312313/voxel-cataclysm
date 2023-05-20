use crate::voxel::ui::{
    styles::{get_text_style},
};

use bevy::prelude::*;

use super::{
    styles::{SUPER_UI}, EndScreenCamera2d,
};

pub fn build_end_screen(mut commands:  Commands, asset_server: Res<AssetServer>) -> Entity {
    let end_screen_entity = commands
        .spawn((
            NodeBundle {
                style: SUPER_UI,
                ..Default::default()
            },
            EndScreenCamera2d,
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
