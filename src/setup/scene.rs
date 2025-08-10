use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;

use crate::components::Ceiling;

pub fn setup_ceiling(mut commands: Commands) {
    commands.spawn((
        Ceiling,
        Transform::from_xyz(0.0, 10.0, 0.0),
        GlobalTransform::default(),
    ));
}

pub fn setup_lights_and_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        affects_lightmapped_meshes: false,
    });

    commands.spawn((
        Camera {
            hdr: true,
            ..default()
        },
        Camera3d { ..default() },
        DistanceFog {
            color: Color::srgb(0.05, 0.05, 0.15),
            falloff: FogFalloff::Linear {
                start: 15.0,
                end: 30.0,
            },
            ..default()
        },
        Bloom::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
    ));
}
