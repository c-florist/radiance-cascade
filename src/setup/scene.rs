use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;

use crate::components::{Ceiling, OrbitCamera};

pub fn setup_ceiling(mut commands: Commands) {
    commands.spawn((
        Ceiling,
        Transform::from_xyz(0.0, 10.0, 0.0),
        GlobalTransform::default(),
    ));
}

pub fn setup_lights_and_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.1, 0.1, 0.3),
        brightness: 0.05,
        ..default()
    });

    let initial_radius = 18.0;
    let initial_angle = std::f32::consts::FRAC_PI_2;

    commands.spawn((
        Camera {
            hdr: true,
            ..default()
        },
        Camera3d { ..default() },
        DistanceFog {
            color: Color::srgb(0.05, 0.05, 0.2),
            falloff: FogFalloff::Exponential { density: 0.12 },
            ..default()
        },
        Bloom {
            intensity: 0.35,
            low_frequency_boost: 0.6,
            low_frequency_boost_curvature: 0.4,
            high_pass_frequency: 0.6,
            ..default()
        },
        Transform::from_xyz(
            initial_radius * initial_angle.cos(),
            7.5,
            initial_radius * initial_angle.sin(),
        )
        .looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
        OrbitCamera {
            radius: initial_radius,
            angle: initial_angle,
        },
    ));
}
