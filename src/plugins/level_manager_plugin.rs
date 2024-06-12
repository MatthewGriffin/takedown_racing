use bevy::{
    app::{Plugin, Update},
    ecs::schedule::OnExit,
};

use crate::{
    states::game_states::GameStates,
    systems::{
        car::spawn_car::spawn_car, lights::setup_lights::setup_lights,
        track::spawn_track::spawn_track,
    },
};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnExit(GameStates::Loading), (spawn_track, spawn_car))
            .add_systems(Update, setup_lights);
    }
}
