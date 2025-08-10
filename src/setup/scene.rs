use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::components::Wall;

pub fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.15),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        Wall,
    ));
}

pub fn setup_lights_and_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
        affects_lightmapped_meshes: false,
    });

    commands.spawn((
        Camera {
            hdr: true,
            ..default()
        },
        Camera3d { ..default() },
        DistanceFog {
            color: Color::srgb(0.0, 0.0, 0.2),
            falloff: FogFalloff::Linear {
                start: 15.0,
                end: 25.0,
            },
            ..default()
        },
        Bloom::default(),
        Transform::from_xyz(0.0, 2.5, 16.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}
