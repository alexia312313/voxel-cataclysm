use crate::voxel::ui::ChatText;
use bevy::prelude::*;
use common::ChatMessages;

pub fn update_chat_text(
    mut text_query: Query<&mut Text, With<ChatText>>,
    chat_messages: Res<ChatMessages>,
) {
    /*if let Ok(player_stats) = cha.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.hp);
        }
    }*/
    for chat_message in chat_messages.message.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", chat_message);
        }
    }
}
