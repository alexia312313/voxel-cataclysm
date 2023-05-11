// The second step is to define an action. What can the AI do, and how does it
// do it? This is the first bit involving Big Brain itself, and there's a few
// pieces you need:
//
// 1. An Action Component. This is just a plain Component we will query
//    against later.
// 2. An ActionBuilder. This is anything that implements the ActionBuilder
//    trait.
// 3. A System that will run Action code.
//
// These actions will be spawned and queued by the game engine when their
// conditions trigger (we'll configure what these are later).
//
// In most cases, the ActionBuilder just attaches the Action component to the
// actor entity. In this case, you can use the derive macro `ActionBuilder`
// to make your Action Component implement the ActionBuilder trait.
// You need your type to implement Clone and Debug (necessary for ActionBuilder)
use bevy::prelude::*;
use big_brain::{
    prelude::{ActionBuilder, ActionState},
    thinker::{ActionSpan, Actor},
};

use super::components::Thirst;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    pub(crate) until: f32,
    pub(crate) per_second: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
pub fn drink_action_system(
    time: Res<Time>,
    mut thirsts: Query<&mut Thirst>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    for (Actor(actor), mut state, drink, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok(mut thirst) = thirsts.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to drink some water!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Drinking...");
                    thirst.thirst -=
                        drink.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    if thirst.thirst <= drink.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        debug!("Done drinking water");
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    debug!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
