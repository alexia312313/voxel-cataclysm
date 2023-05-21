use bevy::prelude::*;

use crate::voxel::{networking::ControlledPlayer, Stats};

use super::{HealthText, ScoreText};

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    stats_query: Query<&Stats, With<ControlledPlayer>>,
) {
    if let Ok(player_stats) = stats_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.score);
        }
    }
}

pub fn update_health_text(
    mut text_query: Query<&mut Text, With<HealthText>>,
    stats_query: Query<&Stats, With<ControlledPlayer>>,
) {
    if let Ok(player_stats) = stats_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", player_stats.hp);
        }
    }
}
