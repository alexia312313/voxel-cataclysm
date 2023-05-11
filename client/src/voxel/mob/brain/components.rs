// First, we define a "Thirst" component and associated system. This is NOT
// THE AI. It's a plain old system that just makes an entity "thirstier" over
// time. This is what the AI will later interact with.
//
// There's nothing special here. It's a plain old Bevy component.
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }
        trace!("Thirst: {}", thirst.thirst);
    }
}
