use bevy::prelude::*;

pub fn track_time(
    time: Res<Time>
){
    let elapsed =
    time.elapsed_seconds_wrapped();

    println!("{}",elapsed)
}