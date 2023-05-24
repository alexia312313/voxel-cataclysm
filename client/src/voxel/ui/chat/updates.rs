use crate::voxel::ui::ChatText;
use bevy::prelude::*;
use common::ChatMessage;

pub fn update_chat_text(
    mut text_query: Query<&mut Text, With<ChatText>>,
    chat_messages: ResMut<ChatMessage>,
) {
    /*if let Ok(player_stats) = cha.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.hp);
        }
    }*/
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", chat_messages.message);
    }
}
