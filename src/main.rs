pub mod asset_collections;
pub mod components;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

use bevy::{app::App, DefaultPlugins};
use plugins::{
    asset_loading_plugin::AssetLoadingPlugin, camera_manager_plugin::CameraManagerPlugin,
    level_manager_plugin::LevelManagerPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraManagerPlugin,
            LevelManagerPlugin,
            AssetLoadingPlugin,
        ))
        .run();
}
