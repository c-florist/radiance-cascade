use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

use crate::components::{Lantern, LanternBob};
use crate::config::LanternConfig;

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
    config: ResMut<LanternConfig>,
) {
    const BASE_EMISSIVE_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);

    for (_, mut material_handle, mut light, mut lantern) in lantern_query.iter_mut() {
        if lantern.is_on {
            lantern.on_timer.tick(time.delta());

            if rng.random_bool(config.flicker_chance) {
                let flicker_amount = lantern.base_intensity * 0.5;
                let flicker = rng.random_range(-flicker_amount..flicker_amount);
                light.intensity = (lantern.base_intensity + flicker).max(0.0);
            } else {
                light.intensity = lantern.base_intensity;
            }

            if let Some(material) = materials.get_mut(&mut material_handle.0) {
                let emissive_factor = if lantern.base_intensity > 0.0 {
                    config.emissive_multiplier * (light.intensity / lantern.base_intensity)
                } else {
                    0.0
                };
                material.emissive = BASE_EMISSIVE_COLOR.to_linear() * emissive_factor;
            }

            if lantern.on_timer.finished() {
                lantern.is_on = false;
                lantern.cooldown.reset();
                light.intensity = 0.0;
                lantern.base_intensity = 0.0;
                if let Some(material) = materials.get_mut(&mut material_handle.0) {
                    material.emissive = Color::BLACK.to_linear();
                }
            }
        } else {
            lantern.cooldown.tick(time.delta());
            if lantern.cooldown.finished() && rng.random_bool(config.on_chance) {
                lantern.is_on = true;
                lantern.on_timer.reset();
                lantern.radiance = rng.random_range(5.0..=15.0);
                let new_intensity = rng.random_range(2000.0..8000.0);
                lantern.base_intensity = new_intensity;
                light.intensity = new_intensity;
                if let Some(material) = materials.get_mut(&mut material_handle.0) {
                    material.emissive =
                        BASE_EMISSIVE_COLOR.to_linear() * config.emissive_multiplier;
                }
            }
        }
    }
}

pub fn lantern_bob_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &LanternBob)>,
    config: Res<LanternConfig>,
) {
    for (mut transform, bob) in query.iter_mut() {
        transform.translation.y = bob.initial_y
            + (time.elapsed_secs() * config.bob_speed + bob.phase_offset).sin()
                * config.bob_amplitude;
    }
}
