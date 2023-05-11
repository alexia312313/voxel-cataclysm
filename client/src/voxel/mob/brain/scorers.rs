// Then, we have something called "Scorers". These are special components that
// run in the background, calculating a "Score" value, which is what Big Brain
// will use to pick which Actions to execute.
//
// Just like with Actions, there is a distinction between Scorer components
// and the ScorerBuilder which will attach those components to the Actor entity.
//
// Again, in most cases, you can use the `ScorerBuilder` derive macro to make your
// Scorer Component act as a ScorerBuilder. You need it to implement Clone and Debug.
use bevy::prelude::*;
use big_brain::{
    prelude::ScorerBuilder,
    scorers::Score,
    thinker::{Actor, ScorerSpan},
};

use super::components::Thirst;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;

// Looks familiar? It's a lot like Actions!
pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Thirsty>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(thirst.thirst / 100.0);
            if thirst.thirst >= 80.0 {
                span.span().in_scope(|| {
                    debug!("Thirst above threshold! Score: {}", thirst.thirst / 100.0)
                });
            }
        }
    }
}
