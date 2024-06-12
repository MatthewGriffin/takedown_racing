use bevy::{asset::Handle, ecs::system::Resource, gltf::Gltf};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct CarsCollection {
    #[asset(path = "cars/car_temp.glb")]
    pub temp_car: Handle<Gltf>,
}
