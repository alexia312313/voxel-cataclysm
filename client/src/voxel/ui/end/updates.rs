use bevy::{prelude::*, app::AppExit};

use crate::voxel::{Stats, networking::{ControlledPlayer,},};

use super::{QuitButton, styles::{PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR}, FinalScoreText, FinalTime, ElapsedTime};

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}


pub fn update_score_text_final(
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
    stats_query: Query<&Stats, With<ControlledPlayer>>,
) {
    
    if let Ok(player_stats) = stats_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.score);
        }
    }
}


pub fn update_time_final(
    mut text_query: Query<&mut Text, With<FinalTime>>,
    time:Query<&ElapsedTime>
) {
    for elapsed in time.iter(){

        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:?}", elapsed);
        }
    }
    
}
