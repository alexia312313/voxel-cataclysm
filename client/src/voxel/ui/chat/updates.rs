use crate::voxel::ui::ChatText;
use bevy::prelude::*;

use common::ChatMessage;
use common::DisplayMessage;

pub fn update_chat_text(
    mut text_query: Query<&mut Text, With<ChatText>>,
    display_message: ResMut<DisplayMessage>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", display_message.message);
    }
}
