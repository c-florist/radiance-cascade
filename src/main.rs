use bevy::prelude::*;
use std::panic;

use crate::config::{LanternConfig, MothConfig};
use crate::setup::{setup_lanterns, setup_lights_and_camera, setup_moths, setup_wall};
use crate::systems::{lantern_power_system, moth_attraction_system, moth_movement_system};

mod components;
mod config;
mod setup;
mod systems;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Radiance cascade".into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(MothConfig::default())
        .insert_resource(LanternConfig::default())
        .add_systems(
            Startup,
            (
                setup_wall,
                setup_lights_and_camera,
                setup_lanterns,
                setup_moths,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                lantern_power_system,
                moth_attraction_system,
                moth_movement_system,
            )
                .chain(),
        )
        .run();
}
