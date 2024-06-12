use bevy::app::Plugin;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};

use crate::{
    asset_collections::{cars_collection::CarsCollection, tracks_collection::TracksCollection},
    states::game_states::GameStates,
};

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameStates>().add_loading_state(
            LoadingState::new(GameStates::Loading)
                .continue_to_state(GameStates::Running)
                .load_collection::<TracksCollection>()
                .load_collection::<CarsCollection>(),
        );
    }
}
