use bevy::{
    core_pipeline::core_3d::Camera3dBundle, ecs::system::Commands, math::Vec3,
    transform::components::Transform, utils::default,
};

use crate::components::main_camera_component::MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}
