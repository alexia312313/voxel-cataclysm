use std::collections::VecDeque;

use bevy::{math::IVec2, prelude::*, utils::HashMap};
use building_blocks::{
    core::{Extent3i, PointN},
    prelude::*,
};

use crate::Player;

pub type ChunkMap = HashMap<IVec2, Entity>;

pub const CHUNK_HEIGHT: i32 = 128;
pub const CHUNK_WIDTH: i32 = 16;
pub const CHUNK_DEPTH: i32 = 16;

pub const DEFAULT_VIEW_DISTANCE: i32 = 24;

#[inline]
pub fn chunk_extent() -> Extent3i {
    Extent3i::from_min_and_shape(
        PointN([0; 3]),
        PointN([CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH]),
    )
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Voxel {
    pub attributes: [u8; 4],
}

#[derive(Default)]
pub struct VoxelWorld {
    pub loaded_chunks: ChunkMap,
}

/// A component tracking the current loading state of a chunk.
pub enum ChunkLoadState {
    Load,   // Chunk needs to be loaded from disk.
    Unload, // Chunk needs to be saved to disk and unloaded.
    //Despawn, // Chunk will be despawned on next frame.
    Generate, // Chunk wasn't generated beforehand and needs to be generated by the worldgen.
    Done,     // Chunk is done loading.
}

struct ChunkSpawnRequest(IVec2);
struct ChunkDespawnRequest(IVec2, Entity);

struct ChunkLoadRequest(Entity);

/// An event signaling that a chunk and its data have finished loading and are ready to be displayed.
pub struct ChunkReadyEvent(pub IVec2, pub Entity);

/// A component describing a chunk.
pub struct Chunk {
    pub pos: IVec2,
    pub block_data: Array3x1<Voxel>,
}

#[derive(Bundle)]
pub struct ChunkDataBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub chunk: Chunk,
}

/// Handles the visibility checking of the currently loaded chunks around the player.
/// This will accordingly emit [`ChunkSpawnRequest`] events for chunks that need to be loaded since they entered the player's view distance and [`ChunkDespawnRequest`] for
/// chunks out of the player's view distance.
fn update_visible_chunks(
    player: Query<(&Transform, &Player)>,
    world: Res<VoxelWorld>,
    mut spawn_requests: EventWriter<ChunkSpawnRequest>,
    mut despawn_requests: EventWriter<ChunkDespawnRequest>,
) {
    if let Ok((transform, _)) = player.single() {
        let pos = global2chunk(transform.translation);

        let mut load_radius_chunks: Vec<IVec2> = Vec::new();

        //todo: sort chunk loading order from closest to farthest from ply view.
        for dx in -DEFAULT_VIEW_DISTANCE..=DEFAULT_VIEW_DISTANCE {
            for dy in -DEFAULT_VIEW_DISTANCE..=DEFAULT_VIEW_DISTANCE {
                if dx.pow(2) + dy.pow(2) >= DEFAULT_VIEW_DISTANCE.pow(2) {
                    continue;
                };

                let chunk_pos = pos + (dx, dy).into();
                if !world.loaded_chunks.contains_key(&chunk_pos) {
                    load_radius_chunks.push(chunk_pos);
                }
            }
        }

        load_radius_chunks.sort_by_key(|a| (a.x.pow(2) + a.y.pow(2)));

        spawn_requests.send_batch(
            load_radius_chunks
                .iter()
                .map(|c| ChunkSpawnRequest(c.clone())),
        );

        for key in world.loaded_chunks.keys() {
            let delta = *key - pos;
            let entity = world.loaded_chunks.get(key).unwrap().clone();
            if delta.x.abs().pow(2) + delta.y.abs().pow(2) > DEFAULT_VIEW_DISTANCE.pow(2) {
                despawn_requests.send(ChunkDespawnRequest(key.clone(), entity));
            }
        }
    }
}

fn create_chunks(
    mut commands: Commands,
    mut spawn_events: EventReader<ChunkSpawnRequest>,
    mut world: ResMut<VoxelWorld>,
) {
    for creation_request in spawn_events.iter() {
        let entity = commands
            .spawn_bundle(ChunkDataBundle {
                transform: Transform::from_translation(chunk2global(creation_request.0)),
                chunk: Chunk {
                    pos: creation_request.0,
                    block_data: Array3x1::fill(chunk_extent().padded(1), Voxel::default()),
                },
                global_transform: Default::default(),
            })
            .insert(ChunkLoadState::Load)
            .id();

        world.loaded_chunks.insert(creation_request.0, entity);
    }
}

//todo: parallelize this.
//todo: run this on the IOTaskPool
/// Loads from disk the chunk data of chunks with a current load state of [`ChunkLoadState::Load`].
/// If the chunk wasn't generated, the [`ChunkLoadState`] of the chunk is set to [`ChunkLoadState::Generate`].
fn load_chunk_data(
    mut chunks: Query<(&mut ChunkLoadState, Entity), Added<Chunk>>,
    mut gen_requests: ResMut<VecDeque<ChunkLoadRequest>>,
) {
    for (mut load_state, entity) in chunks.iter_mut() {
        match *load_state {
            ChunkLoadState::Load => {
                *load_state = ChunkLoadState::Generate;
                gen_requests.push_front(ChunkLoadRequest(entity));
            }
            _ => continue,
        }
    }
}

/// Marks the load state of all chunk that are queued to be unloaded as [`ChunkLoadState::Unload`]
fn prepare_for_unload(
    mut despawn_events: EventReader<ChunkDespawnRequest>,
    mut chunks: Query<&mut ChunkLoadState>,
) {
    for despawn_event in despawn_events.iter() {
        if let Ok(mut load_state) = chunks.get_mut(despawn_event.1) {
            *load_state = ChunkLoadState::Unload;
        }
    }
}

/// Destroys all the chunks that have a load state of [`ChunkLoadState::Unload`]
fn destroy_chunks(
    mut commands: Commands,
    mut world: ResMut<VoxelWorld>,
    chunks: Query<(&Chunk, &ChunkLoadState)>,
) {
    for (chunk, load_state) in chunks.iter() {
        match load_state {
            ChunkLoadState::Unload => {
                let entity = world.loaded_chunks.remove(&chunk.pos).unwrap();
                commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}

fn generate_chunks(
    mut query: Query<(&mut Chunk, &mut ChunkLoadState)>,
    mut gen_requests: ResMut<VecDeque<ChunkLoadRequest>>,
) {
    for _ in 0..(DEFAULT_VIEW_DISTANCE / 2) {
        if let Some(ev) = gen_requests.pop_back() {
            if let Ok((mut data, mut load_state)) = query.get_mut(ev.0) {
                data.block_data.fill_extent(
                    &chunk_extent(),
                    Voxel {
                        attributes: [255; 4],
                    },
                );
                *load_state = ChunkLoadState::Done;
            }
        }
    }
}

fn mark_chunks_ready(
    mut ready_events: EventWriter<ChunkReadyEvent>,
    chunks: Query<(&Chunk, &ChunkLoadState, Entity), Changed<ChunkLoadState>>,
) {
    for (chunk, load_state, entity) in chunks.iter() {
        match load_state {
            ChunkLoadState::Done => ready_events.send(ChunkReadyEvent(chunk.pos, entity)),
            _ => {}
        }
    }
}

pub fn global2chunk(position: Vec3) -> IVec2 {
    IVec2::new(
        position.x.floor() as i32 / CHUNK_WIDTH,
        position.z.floor() as i32 / CHUNK_DEPTH,
    )
}

pub fn globalil2local(pos: IVec3) -> IVec3 {
    IVec3::new(pos.x % CHUNK_WIDTH, pos.y, pos.z % CHUNK_DEPTH)
}

pub fn globali2chunk(position: IVec3) -> IVec2 {
    IVec2::new(position.x / CHUNK_WIDTH, position.z / CHUNK_DEPTH)
}

pub fn chunk2global(chunk_coords: IVec2) -> Vec3 {
    Vec3::new(
        (chunk_coords.x * CHUNK_WIDTH) as f32,
        0.,
        (chunk_coords.y * CHUNK_DEPTH) as f32,
    )
}

pub struct WorldSimulationPlugin;

impl Plugin for WorldSimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(VoxelWorld::default())
            .init_resource::<VecDeque<ChunkLoadRequest>>()
            // internal events
            .add_event::<ChunkSpawnRequest>()
            .add_event::<ChunkDespawnRequest>()
            // public events
            .add_event::<ChunkReadyEvent>()
            // systems
            .add_system(update_visible_chunks.system())
            .add_system(create_chunks.system())
            .add_system(load_chunk_data.system())
            .add_system(generate_chunks.system())
            .add_system(prepare_for_unload.system())
            .add_system(mark_chunks_ready.system())
            .add_system(destroy_chunks.system());
    }
}
