use bevy::prelude::*;
use common::{ChatMessage, Player};

/// prints every char coming in; press enter to echo the full string
pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut writing: Local<bool>,
    mut chat_messages: ResMut<ChatMessage>,
    player_query: Query<&Player>,
) {
    if !*writing {
        if keys.just_pressed(KeyCode::Return) {
            *writing = true;
        }
    }
    if *writing {
        for ev in char_evr.iter() {
            string.push(ev.char);
        }

        if keys.just_pressed(KeyCode::F1) {
            if player_query.get_single().is_err() {
                return;
            }
            println!(" {}", *string);
            chat_messages.message = string.to_string();
            chat_messages.client_id = player_query.get_single().unwrap().id;
            println!("chat_messages: {:?}", chat_messages);
            string.clear();
            *writing = false;
        }
    }
}
