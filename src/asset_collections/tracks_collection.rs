use bevy::{asset::Handle, ecs::system::Resource, gltf::Gltf};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct TracksCollection {
    #[asset(path = "tracks/oval_track.glb")]
    pub oval_track: Handle<Gltf>,
}
