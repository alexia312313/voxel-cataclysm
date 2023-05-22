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
    let random_number = rng.gen_range(100..200) as f32;
    if let Ok((transform, mut timer)) = query.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let player_pos = transform.translation;
            let mob_pos = Vec3::new(
                player_pos.x + random_number,
                190.0,
                player_pos.z + random_number,
            );
            println!("Mob Spawned at {:?}", mob_pos);
            spawn_mob(&mut cmds, &my_assets, mob_pos);
            timer.0.reset();
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
        Mob,
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
