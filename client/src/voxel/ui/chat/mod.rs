use bevy::prelude::*;
use common::ChatMessage;

use crate::GameState;

use self::{ui::text_input, updates::update_chat_text};

mod ui;
mod updates;
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app
            //systems
            .add_systems((text_input, update_chat_text).in_set(OnUpdate(GameState::Game)))
            .insert_resource(ChatMessage {
                message: "".to_string(),
                client_id: 0,
            });
    }
}
