use bevy::prelude::*;
use rand::Rng;

use crate::components::{Lantern, Moth, Velocity, Wall};
use crate::config::MothConfig;

pub fn setup_lanterns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wall_query: Query<&Transform, With<Wall>>,
) {
    const LANTERN_SPACING: f32 = 4.5;
    const WALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
    let mut rng = rand::rng();

    if let Ok(wall_transform) = wall_query.single() {
        let num_x = (WALL_SIZE.x / LANTERN_SPACING).floor() as i32;
        let num_y = (WALL_SIZE.y / LANTERN_SPACING).floor() as i32;

        for i in 0..num_x {
            for j in 0..num_y {
                let x = (i as f32 - num_x as f32 / 2.0 + 0.5) * LANTERN_SPACING;
                let y = (j as f32 - num_y as f32 / 2.0 + 0.5) * LANTERN_SPACING;

                let lantern_glow_color = Color::srgb(1.0, 0.5, 0.0);

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.8, 0.7, 0.6),
                        ..default()
                    })),
                    PointLight {
                        intensity: 0.0,
                        shadows_enabled: true,
                        color: lantern_glow_color,
                        ..default()
                    },
                    Transform::from_xyz(x, y, wall_transform.translation.z + 0.5),
                    Lantern {
                        radiance: 15.0,
                        is_on: false,
                        on_timer: Timer::from_seconds(
                            rng.random_range(30.0..60.0),
                            TimerMode::Once,
                        ),
                        cooldown: Timer::from_seconds(10.0, TimerMode::Once),
                        grid_pos: (i, j),
                    },
                ));
            }
        }
    }
}

pub fn setup_moths(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<MothConfig>,
) {
    let mut rng = rand::rng();

    for _ in 0..config.moth_count {
        commands.spawn((
            Mesh3d(meshes.add(Cone::new(0.05, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
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
                    * config.moth_speed,
            ),
        ));
    }
}
