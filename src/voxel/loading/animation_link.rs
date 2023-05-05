use crate::debug::DebugUISet;
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

#[derive(Resource)]
pub struct PlayerAnimations(Vec<Handle<AnimationClip>>, Handle<AnimationPlayer>);

pub fn set_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut cmds: Commands,
    asset: Res<AssetServer>,
) {
    let mut index = 0;
    for animation_entity in query.iter_mut() {
        index += 1;
        if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity.0) {
            match index {
                1 => {
                    cmds.insert_resource(PlayerAnimations(
                        vec![
                            // idle animation
                            asset.load("models/player/mereo.glb#Animation0"),
                            // walk animation
                            asset.load("models/player/mereo.glb#Animation1"),
                            // hit animation
                            asset.load("models/player/mereo.glb#Animation2"),
                        ],
                        animation_player,
                    ));
                }
                2 => print!("TODO slime animations"),
            }
        }
    }
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}
fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub struct AnimationLinkPlugin;

impl Plugin for AnimationLinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (link_animations, set_animations)
                .chain()
                .in_base_set(CoreSet::Update)
                .after(DebugUISet::Display),
        );
    }
}
