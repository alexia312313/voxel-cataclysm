use crate::GameState;
use bevy::prelude::*;

pub mod actions;
pub mod components;
pub mod scorers;
pub mod thinkers;

use actions::*;
use big_brain::{BigBrainPlugin, BigBrainSet};
use components::*;
use scorers::*;
use thinkers::*;

pub struct BrainHandlerPlugin;

impl Plugin for BrainHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_systems((init_entities, aggro_system).in_set(OnUpdate(GameState::Game)))
            .configure_set(
                BigBrainActionsSet
                    .run_if(in_state(GameState::Game))
                    .in_set(BigBrainSet::Actions),
            )
            .configure_set(
                BigBrainScorersSet
                    .run_if(in_state(GameState::Game))
                    .in_set(BigBrainSet::Scorers),
            )
            // Big Brain has specific stages for Scorers and Actions. If
            // determinism matters a lot to you, you should add your action and
            // scorer systems to these stages.
            .add_system(attack_action_system.in_set(BigBrainActionsSet))
            .add_system(aggroed_scorer_system.in_set(BigBrainScorersSet));
    }
}

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct BigBrainActionsSet;

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct BigBrainScorersSet;
