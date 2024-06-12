use bevy::{
    ecs::system::{Local, Query},
    log::info,
    pbr::DirectionalLight,
};

pub fn setup_lights(mut lights_query: Query<&mut DirectionalLight>, mut do_once: Local<bool>) {
    if !*do_once {
        for mut light in lights_query.iter_mut() {
            if !light.shadows_enabled {
                info!("Enabling Shadows on Directional Light");
                light.shadows_enabled = true;
            }
            *do_once = true;
        }
    }
}
