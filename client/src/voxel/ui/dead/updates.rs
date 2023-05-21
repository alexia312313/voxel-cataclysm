use bevy::prelude::*;

use crate::voxel::{ui::{ScoreText, end::FinalScoreText}, Stats, networking::ControlledPlayer};

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    stats_query: Query<&Stats, With<ControlledPlayer>>,
) {
    if let Ok(player_stats) = stats_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.score);
        }
    }
}pub fn update_score_text_dead(
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
    stats_query: Query<&Stats, With<ControlledPlayer>>,
) {
    if let Ok(player_stats) = stats_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.score);
        }
    }
}