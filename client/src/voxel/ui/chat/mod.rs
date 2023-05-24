use bevy::prelude::*;

use crate::GameState;

use self::ui::text_input;

mod ui;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app
            //systems
            .add_system((text_input).in_set(OnUpdate(GameState::Game)));
    }
}
