use bevy::{prelude::*, time, utils::HashMap};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, LockedAxes, RigidBody};
use rand::Rng;

use crate::voxel::{
    animation::{AnimationController, Animations},
    loading::MyAssets,
    mob::Mob,
    networking::ControlledPlayer,
    player::MobSpawnTimer,
    Stats,
};

pub fn spawn_mobs(
    mut cmds: Commands,
    my_assets: Res<MyAssets>,
    mut query: Query<(&Transform, &mut MobSpawnTimer), With<ControlledPlayer>>,
    time: Res<time::Time>,
) {
    // random number from 100 to 200
    let mut rng = rand::thread_rng();
    // random bool that is 50% false, 50% true
    let random_bool = rng.gen_bool(0.5);
    let random_number = rng.gen_range(50..100) as f32;
    // if random_bool is false, make the random number negative
    let random_number = if random_bool {
        random_number
    } else {
        -random_number
    };

    if let Ok((transform, mut timer)) = query.get_single_mut() {
        if timer.current_mobs < timer.max_mobs {
            timer.get_timer.tick(time.delta());
            if timer.get_timer.just_finished() {
                let player_pos = transform.translation;
                let mob_pos = Vec3::new(
                    player_pos.x + random_number,
                    190.0,
                    player_pos.z + random_number,
                );
                println!("Mob Spawned at {:?}", mob_pos);
                spawn_mob(&mut cmds, &my_assets, mob_pos);
                timer.current_mobs += 1;
                timer.get_timer.reset();
            }
        }
    }
}

pub fn spawn_mob(cmds: &mut Commands, _my_assets: &Res<MyAssets>, pos: Vec3) {
    let mut map = HashMap::new();
    map.insert(
        "walk".to_string(),
        _my_assets.slime_animation_walking.clone(),
    );

    cmds.spawn((
        Mob(generate_id(10)),
        Stats {
            hp: 20,
            max_hp: 20,
            attack: 10,
            speed: 5.0,
            score: 10,
        },
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        TransformBundle {
            local: Transform::from_xyz(pos.x, pos.y, pos.z).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
    ))
    .with_children(|mob| {
        mob.spawn(SceneBundle {
            scene: _my_assets.slime.clone(),
            transform: Transform::IDENTITY.looking_to(Vec3::Z, Vec3::Y),
            ..default()
        });
    })
    .insert(AnimationController { done: false })
    .insert(Animations(map))
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(ActiveEvents::COLLISION_EVENTS);
}

pub fn generate_id(length: usize) -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    let id: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());
            chars[index]
        })
        .collect();
    id
}
