use crate::components::{Moth, Velocity};
use crate::constants::{
    ALIGNMENT_WEIGHT, COHESION_WEIGHT, MOTH_SPEED, PERCEPTION_RADIUS, SEPARATION_WEIGHT,
};
use bevy::prelude::*;

pub fn flocking_system(mut query: Query<(Entity, &Transform, &mut Velocity), With<Moth>>) {
    // Collect all moth data into a vector of owned types. This avoids holding a borrow on the query
    // and allows us to iterate over it mutably later.
    let moth_data: Vec<(Entity, Transform, Velocity)> = query
        .iter()
        .map(|(entity, transform, velocity)| (entity, *transform, velocity.clone()))
        .collect();

    for (entity, transform, mut velocity) in query.iter_mut() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut local_flockmates = 0;

        // Use the collected data for calculations
        for (other_entity, other_transform, other_velocity) in &moth_data {
            // Don't compare a moth to itself
            if entity == *other_entity {
                continue;
            }

            let distance = transform.translation.distance(other_transform.translation);

            if distance > 0.0 && distance < PERCEPTION_RADIUS {
                // Separation: Steer away from neighbors to avoid crowding
                separation +=
                    (transform.translation - other_transform.translation) / distance.powi(2);

                // Alignment: Steer towards the average heading of neighbors
                alignment += other_velocity.0;

                // Cohesion: Steer towards the average position of neighbors
                cohesion += other_transform.translation;

                local_flockmates += 1;
            }
        }

        if local_flockmates > 0 {
            // Calculate the average alignment and cohesion
            let avg_alignment = alignment / local_flockmates as f32;
            let avg_cohesion = cohesion / local_flockmates as f32;

            // Calculate steering force, using normalize_or_zero to prevent panics
            let alignment_force = (avg_alignment.normalize_or_zero() * MOTH_SPEED) - velocity.0;
            let cohesion_force = (avg_cohesion - transform.translation).normalize_or_zero()
                * MOTH_SPEED
                - velocity.0;

            // Apply weights to the forces
            velocity.0 += separation * SEPARATION_WEIGHT;
            velocity.0 += alignment_force * ALIGNMENT_WEIGHT;
            velocity.0 += cohesion_force * COHESION_WEIGHT;
        }

        // Clamp velocity to max speed and apply it
        velocity.0 = velocity.0.normalize_or_zero() * MOTH_SPEED;
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
