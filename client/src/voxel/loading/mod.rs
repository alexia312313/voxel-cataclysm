use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingHandlerPlugin;

impl Plugin for LoadingHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // Player model
    #[asset(path = "models/player/mereo.gltf#Scene0")]
    pub player: Handle<Scene>,
    // Items models
    #[asset(path = "models/item/purple-sword.gltf#Scene0")]
    pub sword: Handle<Scene>,
    // Mobs models
    #[asset(path = "models/mob/slime.gltf#Scene0")]
    pub slime: Handle<Scene>,
    // Player animations
    #[asset(path = "models/player/mereo.gltf#Animation0")]
    pub player_animation_hit: Handle<AnimationClip>,
    #[asset(path = "models/player/mereo.gltf#Animation2")]
    pub player_animation_walk: Handle<AnimationClip>,
    #[asset(path = "models/player/mereo.gltf#Animation1")]
    pub player_animation_idle: Handle<AnimationClip>,
    // Mobs animations
    #[asset(path = "models/mob/slime.gltf#Animation0")]
    pub slime_animation_walking: Handle<AnimationClip>,
    // Portal
    #[asset(path = "models/item/Portal.gltf#Scene0")]
    pub end_portal: Handle<Scene>,
    // Arrow 
    #[asset(path = "models/item/arrowx100.gltf#Scene0")]
    pub arrow: Handle<Scene>
}
