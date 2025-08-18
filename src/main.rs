use bevy::prelude::*;
use bevy_rand::prelude::{EntropyPlugin, WyRand};
use std::panic;

use crate::config::{LanternConfig, MothConfig};
use crate::resources::{SpatialIndex, TouchState};
use crate::setup::{
    setup_ceiling, setup_lantern_index, setup_lanterns, setup_lights_and_camera, setup_moths,
};
use crate::systems::{
    camera_control_system, enforce_boundary_system, lantern_bob_system, lantern_power_system,
    moth_attraction_system, moth_collision_system, moth_movement_system, moth_wander_system,
};

mod components;
mod config;
mod resources;
mod setup;
mod systems;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Phototaxis".into(),
                    canvas: Some("#bevy".to_owned()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            EntropyPlugin::<WyRand>::default(),
        ))
        .init_resource::<SpatialIndex>()
        .init_resource::<TouchState>()
        .insert_resource(MothConfig::default())
        .insert_resource(LanternConfig::default())
        .insert_resource(ClearColor(Color::srgb(0.01, 0.01, 0.08)))
        .add_systems(
            Startup,
            (
                setup_ceiling,
                setup_lights_and_camera,
                setup_lanterns,
                setup_moths,
                setup_lantern_index,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    moth_wander_system,
                    moth_attraction_system,
                    moth_collision_system,
                    enforce_boundary_system,
                )
                    .chain(),
                moth_movement_system,
                lantern_power_system,
                lantern_bob_system,
                camera_control_system,
            ),
        )
        .run();
}
