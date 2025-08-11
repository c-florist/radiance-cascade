use crate::components::{Lantern, LanternBob};
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

pub fn lantern_power_system(
    mut lantern_query: Query<
        (
            &Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut PointLight,
            &mut Lantern,
        ),
        With<Lantern>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut rng: GlobalEntropy<WyRand>,
) {
    const TURN_ON_CHANCE: f64 = 0.01;

    let mut grid_state: Vec<((i32, i32), bool)> = lantern_query
        .iter()
        .map(|(_, _, _, lantern)| (lantern.grid_pos, lantern.is_on))
        .collect();

    for (_, mut material, mut light, mut lantern) in lantern_query.iter_mut() {
        if lantern.is_on {
            lantern.on_timer.tick(time.delta());
            if lantern.on_timer.finished() {
                lantern.is_on = false;
                lantern.cooldown.reset();
                light.intensity = 0.0;
                if let Some(material) = materials.get_mut(&mut material.0) {
                    material.emissive = Color::BLACK.to_linear();
                }
            }
        } else {
            lantern.cooldown.tick(time.delta());
            if lantern.cooldown.finished() && rng.random_bool(TURN_ON_CHANCE) {
                lantern.is_on = true;
                lantern.on_timer.reset();
                lantern.radiance = rng.random_range(5.0..=15.0);
                light.intensity = rng.random_range(1000.0..8000.0);
                if let Some(material) = materials.get_mut(&mut material.0) {
                    material.emissive = Color::srgb(1.0, 0.5, 0.0).to_linear() * 100.0;
                }
                if let Some(grid_lantern) = grid_state
                    .iter_mut()
                    .find(|(pos, _)| *pos == lantern.grid_pos)
                {
                    grid_lantern.1 = true;
                }
            }
        }
    }
}

pub fn lantern_bob_system(time: Res<Time>, mut query: Query<(&mut Transform, &LanternBob)>) {
    const BOB_SPEED: f32 = 1.15;
    const BOB_AMPLITUDE: f32 = 0.25;

    for (mut transform, bob) in query.iter_mut() {
        transform.translation.y = bob.initial_y
            + (time.elapsed_secs() * BOB_SPEED + bob.phase_offset).sin() * BOB_AMPLITUDE;
    }
}
