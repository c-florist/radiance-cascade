use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::components::{Moth, Velocity};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Radiance Cascade".into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, setup_moths).chain())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
        affects_lightmapped_meshes: false,
    });

    // Wall
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.15),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
    ));

    let lantern_glow_color = Color::srgb(1.0, 0.5, 0.0);

    // Lantern
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            emissive: lantern_glow_color.to_linear() * 100.0,
            ..default()
        })),
        PointLight {
            intensity: 100_000.0,
            shadows_enabled: true,
            color: lantern_glow_color,
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 0.5),
    ));

    // Camera
    commands.spawn((
        Camera {
            hdr: true,
            ..default()
        },
        Camera3d { ..default() },
        Bloom::default(),
        Transform::from_xyz(0.0, 2.5, 8.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}

pub fn setup_moths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const MOTH_COUNT: u32 = 150;
    const MOTH_SPEED: f32 = 2.5;

    let mut rng = rand::rng();

    for _ in 0..MOTH_COUNT {
        commands.spawn((
            Mesh3d(meshes.add(Cone::new(0.05, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb(0.9, 0.9, 0.8))),
            Transform::from_xyz(
                rng.random_range(-5.0..5.0),
                rng.random_range(0.5..4.0),
                rng.random_range(1.0..5.0),
            ),
            Moth,
            Velocity(
                Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                )
                .normalize()
                    * MOTH_SPEED,
            ),
        ));
    }
}
