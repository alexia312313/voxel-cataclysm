// Now that we have all that defined, it's time to add a Thinker to an entity!
// The Thinker is the actual "brain" behind all the AI. Every entity you want
// to have AI behavior should have one *or more* Thinkers attached to it.
use bevy::prelude::*;
use big_brain::{prelude::FirstToScore, thinker::Thinker};

use super::{actions::Drink, components::Thirst, scorers::Thirsty};

pub fn init_entities(mut cmd: Commands) {
    // Create the entity and throw the Thirst component in there. Nothing special here.
    cmd.spawn((
        Thirst::new(75.0, 2.0),
        Thinker::build()
            .label("My Thinker")
            .picker(FirstToScore { threshold: 0.8 })
            // Technically these are supposed to be ActionBuilders and
            // ScorerBuilders, but our Clone impls simplify our code here.
            .when(
                Thirsty,
                Drink {
                    until: 70.0,
                    per_second: 5.0,
                },
            ),
    ));
}
