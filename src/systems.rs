use crate::components::{LandedTimer, Lantern, Moth, Velocity};
use crate::constants::{
    ALIGNMENT_WEIGHT, ATTRACTION_WEIGHT, COHESION_WEIGHT, LANDED_DURATION_SECS, LANDING_CHANCE,
    LANDING_DISTANCE, MOTH_SPEED, PERCEPTION_RADIUS, SEPARATION_WEIGHT,
};
use bevy::prelude::*;
use rand::Rng;

/// This system manages the flocking behavior of moths that are currently flying.
pub fn flocking_system(
    // We query for moths that are specifically NOT landed.
    mut moth_query: Query<(Entity, &Transform, &mut Velocity), (With<Moth>, Without<LandedTimer>)>,
    lantern_query: Query<(&Transform, &Lantern)>,
) {
    let moth_data: Vec<(Entity, Transform, Velocity)> = moth_query
        .iter()
        .map(|(entity, transform, velocity)| (entity, *transform, velocity.clone()))
        .collect();

    let lanterns: Vec<(&Transform, &Lantern)> = lantern_query.iter().collect();

    for (entity, transform, mut velocity) in moth_query.iter_mut() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut attraction = Vec3::ZERO;
        let mut local_flockmates = 0;

        // --- Flocking Calculations ---
        for (other_entity, other_transform, other_velocity) in &moth_data {
            if entity == *other_entity {
                continue;
            }

            let distance = transform.translation.distance(other_transform.translation);
            if distance > 0.0 && distance < PERCEPTION_RADIUS {
                separation +=
                    (transform.translation - other_transform.translation) / distance.powi(2);
                alignment += other_velocity.0;
                cohesion += other_transform.translation;
                local_flockmates += 1;
            }
        }

        // --- Attraction Calculation ---
        if !lanterns.is_empty() {
            let (closest_lantern_transform, lantern) = lanterns
                .iter()
                .min_by(|(a, _), (b, _)| {
                    let dist_a = transform.translation.distance(a.translation);
                    let dist_b = transform.translation.distance(b.translation);
                    dist_a.partial_cmp(&dist_b).unwrap()
                })
                .unwrap();

            let direction_to_lantern =
                (closest_lantern_transform.translation - transform.translation).normalize_or_zero();
            attraction = direction_to_lantern * lantern.radiance;
        }

        // --- Apply Forces ---
        if local_flockmates > 0 {
            let avg_alignment = alignment / local_flockmates as f32;
            let avg_cohesion = cohesion / local_flockmates as f32;

            let alignment_force = (avg_alignment.normalize_or_zero() * MOTH_SPEED) - velocity.0;
            let cohesion_force = (avg_cohesion - transform.translation).normalize_or_zero()
                * MOTH_SPEED
                - velocity.0;

            velocity.0 += separation * SEPARATION_WEIGHT;
            velocity.0 += alignment_force * ALIGNMENT_WEIGHT;
            velocity.0 += cohesion_force * COHESION_WEIGHT;
        }

        velocity.0 += attraction * ATTRACTION_WEIGHT;

        // Clamp velocity to max speed
        velocity.0 = velocity.0.normalize_or_zero() * MOTH_SPEED;
    }
}

/// This system manages the state of moths, allowing them to land on and take off from lanterns.
pub fn moth_landing_system(
    mut commands: Commands,
    time: Res<Time>,
    lantern_query: Query<&Transform, With<Lantern>>,
    mut moth_query: Query<
        (Entity, &Transform, &mut Velocity, Option<&mut LandedTimer>),
        With<Moth>,
    >,
) {
    let mut rng = rand::rng();
    let lanterns: Vec<&Transform> = lantern_query.iter().collect();

    if lanterns.is_empty() {
        return; // No lanterns, no landing
    }

    for (moth_entity, moth_transform, mut velocity, landed_timer_opt) in &mut moth_query {
        if let Some(mut landed_timer) = landed_timer_opt {
            // --- Moth is currently landed, check if it should take off ---
            landed_timer.0.tick(time.delta());
            if landed_timer.0.finished() {
                // Timer finished, remove the landed state and give it a new velocity
                commands.entity(moth_entity).remove::<LandedTimer>();
                velocity.0 = Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                )
                .normalize_or_zero()
                    * MOTH_SPEED;
            }
        } else {
            // --- Moth is currently flying, check if it should land ---
            // Find the closest lantern
            let closest_lantern_transform = lanterns
                .iter()
                .min_by(|a, b| {
                    let dist_a = moth_transform.translation.distance(a.translation);
                    let dist_b = moth_transform.translation.distance(b.translation);
                    dist_a.partial_cmp(&dist_b).unwrap()
                })
                .unwrap();

            let distance_to_lantern = moth_transform
                .translation
                .distance(closest_lantern_transform.translation);

            // Check if the moth is within landing distance and has a chance to land
            if distance_to_lantern < LANDING_DISTANCE && rng.random_bool(LANDING_CHANCE) {
                // Stop the moth and add the landed timer component
                velocity.0 = Vec3::ZERO;
                commands
                    .entity(moth_entity)
                    .insert(LandedTimer(Timer::from_seconds(
                        LANDED_DURATION_SECS,
                        TimerMode::Once,
                    )));
            }
        }
    }
}

pub fn move_moths_system(
    mut query: Query<(&mut Transform, &Velocity), With<Moth>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
        if velocity.0 != Vec3::ZERO {
            transform.look_to(velocity.0, Vec3::Y);
        }
    }
}
