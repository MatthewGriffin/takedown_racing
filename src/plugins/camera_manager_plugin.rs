use bevy::app::{Plugin, Startup};

use crate::systems::camera::setup_camera::setup_camera;

pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera);
    }
}
