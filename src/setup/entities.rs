use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

use crate::components::{Ceiling, Lantern, Moth, Velocity};
use crate::config::MothConfig;

pub fn setup_lanterns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ceiling_query: Query<&Transform, With<Ceiling>>,
    mut rng: GlobalEntropy<WyRand>,
) {
    const LANTERN_SPACING: f32 = 5.0;
    const ROOM_RADIUS: f32 = 10.0;

    if let Ok(ceiling_transform) = ceiling_query.single() {
        let num_x = (ROOM_RADIUS * 2.0 / LANTERN_SPACING).floor() as i32;
        let num_z = (ROOM_RADIUS * 2.0 / LANTERN_SPACING).floor() as i32;

        for i in 0..num_x {
            for j in 0..num_z {
                let x = (i as f32 - num_x as f32 / 2.0 + 0.5) * LANTERN_SPACING;
                let z = (j as f32 - num_z as f32 / 2.0 + 0.5) * LANTERN_SPACING;
                let y = ceiling_transform.translation.y - rng.random_range(1.0..5.0);

                let lantern_glow_color = Color::srgb(1.0, 0.5, 0.0);

                let mut cooldown = Timer::from_seconds(10.0, TimerMode::Once);
                cooldown.tick(cooldown.duration());

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
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
                    Transform::from_xyz(x, y, z),
                    Lantern {
                        radiance: 0.0,
                        is_on: false,
                        on_timer: Timer::from_seconds(
                            rng.random_range(5.0..=20.0),
                            TimerMode::Once,
                        ),
                        cooldown,
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
    mut rng: GlobalEntropy<WyRand>,
) {
    for _ in 0..config.moth_count {
        commands.spawn((
            Mesh3d(meshes.add(Cone::new(0.05, 0.1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                emissive: Color::srgb(1.0, 1.0, 1.0).to_linear() * 2.0,
                ..default()
            })),
            Transform::from_xyz(
                rng.random_range(-5.0..5.0),
                rng.random_range(1.0..4.0),
                rng.random_range(-5.0..5.0),
            ),
            Moth,
            Velocity(
                Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                )
                .normalize_or_zero()
                    * config.moth_speed,
            ),
        ));
    }
}
