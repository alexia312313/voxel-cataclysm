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

use crate::voxel::{Attacked, Stats};

use super::components::Aggro;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack {
    pub(crate) until: f32,
    pub(crate) per_second: f32,
}

// Action systems execute according to a state machine, where the states are
// labeled by ActionState.
pub fn attack_action_system(
    time: Res<Time>,
    stats_query: Query<&Stats>,
    mut aggros: Query<(&mut Aggro, Entity)>,
    mut cmds: Commands,
    // We execute actions by querying for their associated Action Component
    // (Attack in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (Actor(actor), mut state, attack, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((mut aggro, actor_entity)) = aggros.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("hehehe you are ded! player");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Attacking...");

                    let target_entity = aggro.target;
                    if transform_query.get(target_entity).is_err() {
                        return;
                    }
                    let target = *transform_query.get(target_entity).unwrap();
                    let mut actor = transform_query.get_mut(actor_entity).unwrap();
                    let actor_stats = stats_query.get(actor_entity).unwrap();
                    let target_pos = target.translation;
                    let move_forward = actor.forward();
                    let distance_to_target = target_pos.distance(actor.translation);

                    debug!("target: {:?}, actor: {:?}", target_pos, actor.translation);

                    // look at the target
                    actor.look_at(target_pos, Vec3::Y);

                    // if the target is too far away, slowly lose aggro
                    if distance_to_target > 50.0 {
                        aggro.aggro -=
                            attack.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    }

                    if distance_to_target < 3.0 {
                        cmds.entity(target_entity).insert(Attacked {
                            damage: actor_stats.attack,
                        });
                    } else {
                        // move forward
                        actor.translation +=
                            move_forward * actor_stats.speed * time.delta_seconds();
                    }

                    if aggro.aggro <= attack.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        debug!("Done attacking player!");
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
