use bevy::{
    asset::Assets,
    ecs::system::{Commands, Res},
    gltf::Gltf,
    prelude::default,
    scene::SceneBundle,
};

use crate::asset_collections::cars_collection::CarsCollection;

pub fn spawn_car(
    mut commands: Commands,
    cars_collection: Res<CarsCollection>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let Some(track) = gltf_assets.get(&cars_collection.temp_car) else {
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
