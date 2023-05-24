use bevy::prelude::*;

/// prints every char coming in; press enter to echo the full string
pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut writing: Local<bool>,
) {
    if !*writing {
        if keys.just_pressed(KeyCode::Return) {
            *writing = true;
        }
    }
    if *writing {

        for ev in char_evr.iter() {
            println!("Got char: '{}'", ev.char);
            string.push(ev.char);
        }

        if keys.just_pressed(KeyCode::F1) {
            println!(" {}", *string);
            string.clear();
            *writing = false;
        }
    }
}
