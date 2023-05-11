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
            .add_startup_system(init_entities)
            .add_system(thirst_system)
            .configure_set(
                BigBrainActionsSet
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(BigBrainSet::Actions),
            )
            .configure_set(
                BigBrainScorersSet
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(BigBrainSet::Scorers),
            )
            // Big Brain has specific stages for Scorers and Actions. If
            // determinism matters a lot to you, you should add your action and
            // scorer systems to these stages.
            .add_system(drink_action_system.in_set(BigBrainActionsSet))
            .add_system(thirsty_scorer_system.in_set(BigBrainScorersSet))
            .run();
    }
}

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct BigBrainActionsSet;

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemSet)]
pub struct BigBrainScorersSet;
