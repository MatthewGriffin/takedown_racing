use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res},
    gltf::Gltf,
    prelude::default,
    scene::SceneBundle,
};

use crate::asset_collections::tracks_collection::TracksCollection;

pub fn spawn_track(
    mut commands: Commands,
    tracks_collection: Res<TracksCollection>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let Some(track) = gltf_assets.get(&tracks_collection.oval_track) else {
        return;
    };

    let Some(scene_handler) = track.scenes.first() else {
        return;
    };

    commands.spawn(SceneBundle {
        scene: scene_handler.clone(),
        ..default()
    });
}
