use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

//mod animation_link;
pub struct LodingHandlerPlugin;

impl Plugin for LodingHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // Models
    #[asset(path = "models/player/mereo.gltf#Scene0")]
    pub player: Handle<Scene>,
    #[asset(path = "models/player/purple-sword.gltf#Scene0")]
    pub sword: Handle<Scene>,
    #[asset(path = "models/mob/slime.gltf#Scene0")]
    pub slime: Handle<Scene>,
    // Animations
    #[asset(path = "models/player/mereo.gltf#Animation0")]
    pub player_animation_hit: Handle<AnimationClip>,
    #[asset(path = "models/player/mereo.gltf#Animation2")]
    pub player_animation_walking: Handle<AnimationClip>,
    #[asset(path = "models/player/mereo.gltf#Animation1")]
    pub player_animation_idle: Handle<AnimationClip>,
    #[asset(path = "models/mob/slime.gltf#Animation0")]
    pub slime_animation_walking: Handle<AnimationClip>,
}
