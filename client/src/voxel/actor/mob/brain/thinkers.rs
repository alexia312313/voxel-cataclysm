// Now that we have all that defined, it's time to add a Thinker to an entity!
// The Thinker is the actual "brain" behind all the AI. Every entity you want
// to have AI behavior should have one *or more* Thinkers attached to it.
use super::{actions::Attack, components::Aggro, scorers::Aggroed};
use bevy::prelude::*;
use big_brain::{prelude::FirstToScore, thinker::Thinker};
use common::Mob;

pub fn init_entities(mut cmd: Commands, query: Query<Entity, Added<Mob>>) {
    for entity in query.iter() {
        cmd.entity(entity).insert((
            Aggro::new(90.0, 2.0, Entity::PLACEHOLDER),
            Thinker::build()
                .label("My Thinker")
                .picker(FirstToScore { threshold: 0.8 })
                // Technically these are supposed to be ActionBuilders and
                // ScorerBuilders, but our Clone impls simplify our code here.
                .when(
                    Aggroed,
                    Attack {
                        until: 70.0,
                        per_second: 5.0,
                    },
                ),
        ));
    }
}
