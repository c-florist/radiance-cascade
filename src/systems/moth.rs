use crate::components::{Lantern, Moth, Velocity};
use crate::config::MothConfig;
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

pub fn moth_wander_system(
    mut moth_query: Query<&mut Velocity, With<Moth>>,
    moth_config: Res<MothConfig>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for mut velocity in moth_query.iter_mut() {
        // If the moth has stopped, give it a new random direction.
        if velocity.0 == Vec3::ZERO {
            velocity.0 = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            )
            .normalize_or_zero()
                * moth_config.moth_speed;
        }

        let jitter = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize_or_zero()
            * 0.15;

        velocity.0 = (velocity.0 + jitter).normalize_or_zero() * moth_config.moth_speed;
    }
}

pub fn moth_attraction_system(
    moth_config: Res<MothConfig>,
    mut moth_query: Query<(&Transform, &mut Velocity), With<Moth>>,
    lantern_query: Query<(&Transform, &Lantern)>,
    time: Res<Time>,
) {
    let active_lanterns: Vec<_> = lantern_query
        .iter()
        .filter(|(_, lantern)| lantern.is_on)
        .collect();

    if active_lanterns.is_empty() {
        return;
    }

    for (moth_transform, mut velocity) in moth_query.iter_mut() {
        let mut total_attraction_force = Vec3::ZERO;

        for (lantern_transform, lantern) in &active_lanterns {
            let to_lantern = lantern_transform.translation - moth_transform.translation;
            let dist_sq = to_lantern.length_squared();

            if dist_sq < moth_config.view_radius.powi(2) {
                let strength = lantern.radiance / (dist_sq + 1.0);
                total_attraction_force += to_lantern.normalize_or_zero() * strength;
            }
        }

        if total_attraction_force.length_squared() > 0.0 {
            let acceleration = total_attraction_force * 0.0001;
            velocity.0 += acceleration * time.delta_secs();
        }
    }
}

pub fn moth_movement_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Moth>>,
    time: Res<Time>,
    moth_config: Res<MothConfig>,
) {
    for (mut transform, mut velocity) in &mut query {
        velocity.0 = velocity.0.clamp_length_max(moth_config.moth_speed);
        transform.translation += velocity.0 * time.delta_secs();
        if velocity.0 != Vec3::ZERO {
            transform.look_to(velocity.0, Vec3::Y);
        }
    }
}

pub fn moth_collision_system(
    mut moth_query: Query<(&mut Transform, &mut Velocity), With<Moth>>,
    lantern_query: Query<&Transform, (With<Lantern>, Without<Moth>)>,
) {
    for (mut moth_transform, mut velocity) in moth_query.iter_mut() {
        for lantern_transform in lantern_query.iter() {
            let distance = moth_transform
                .translation
                .distance(lantern_transform.translation);
            if distance < 0.5 {
                let direction = (moth_transform.translation - lantern_transform.translation)
                    .normalize_or_zero();
                velocity.0 = direction * velocity.0.length();
                moth_transform.translation = lantern_transform.translation + direction * 0.5;
            }
        }
    }
}
