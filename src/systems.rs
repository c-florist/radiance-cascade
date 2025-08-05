use crate::components::{LandedTimer, Lantern, Moth, Velocity};
use crate::config::FlockingConfig;
use bevy::prelude::*;
use rand::Rng;

pub fn flocking_system(
    config: Res<FlockingConfig>,
    // TODO: Refactor out ParamSet queries
    mut queries: ParamSet<(
        Query<(Entity, &Transform, &mut Velocity), (With<Moth>, Without<LandedTimer>)>,
        Query<(Entity, &Transform, &Velocity), With<Moth>>,
    )>,
    lanterns: Query<(&Transform, &Lantern)>,
) {
    let flock_snapshot: Vec<(Entity, Transform, Velocity)> = queries
        .p1()
        .iter()
        .map(|(entity, transform, velocity)| (entity, *transform, *velocity))
        .collect();

    let lantern_snapshot: Vec<(&Transform, &Lantern)> = lanterns.iter().collect();

    for (moth_entity, moth_transform, mut velocity) in queries.p0().iter_mut() {
        let (separation, alignment, cohesion, local_flockmates) =
            calculate_flocking_forces(moth_entity, moth_transform, &flock_snapshot, &config);

        let attraction = calculate_attraction_force(moth_transform, &lantern_snapshot);

        let mut final_velocity = velocity.0;

        if local_flockmates > 0 {
            let avg_alignment = alignment / local_flockmates as f32;
            let avg_cohesion = cohesion / local_flockmates as f32;

            let alignment_steering =
                (avg_alignment.normalize_or_zero() * config.moth_speed) - final_velocity;
            let cohesion_steering = (avg_cohesion - moth_transform.translation).normalize_or_zero()
                * config.moth_speed
                - final_velocity;

            final_velocity += separation * config.separation_weight;
            final_velocity += alignment_steering * config.alignment_weight;
            final_velocity += cohesion_steering * config.cohesion_weight;
        }

        final_velocity += attraction * config.attraction_weight;
        velocity.0 = final_velocity.normalize_or_zero() * config.moth_speed;
    }
}

/// Calculate the boids for each moth in the "flock"
fn calculate_flocking_forces(
    current_moth_entity: Entity,
    current_moth_transform: &Transform,
    flock_snapshot: &[(Entity, Transform, Velocity)],
    config: &FlockingConfig,
) -> (Vec3, Vec3, Vec3, u32) {
    // Point away from neighbours to avoid crowding
    let mut separation = Vec3::ZERO;
    // Average velocity/heading of neighbours
    let mut alignment = Vec3::ZERO;
    // Average position of neighbours
    let mut cohesion = Vec3::ZERO;
    let mut local_flockmates = 0;

    for (other_moth_entity, other_moth_transform, other_moth_velocity) in flock_snapshot {
        if current_moth_entity == *other_moth_entity {
            continue;
        }

        let distance = current_moth_transform
            .translation
            .distance(other_moth_transform.translation);

        if distance > 0.0 && distance < config.perception_radius {
            separation += (current_moth_transform.translation - other_moth_transform.translation)
                / distance.powi(2);
            alignment += other_moth_velocity.0;
            cohesion += other_moth_transform.translation;
            local_flockmates += 1;
        }
    }
    (separation, alignment, cohesion, local_flockmates)
}

/// Given a moth's position, find the closest lantern and calculate the
/// attraction force using the lantern's radiance as scale factor.
fn calculate_attraction_force(
    moth_transform: &Transform,
    lantern_snapshot: &[(&Transform, &Lantern)],
) -> Vec3 {
    if lantern_snapshot.is_empty() {
        return Vec3::ZERO;
    }

    let (closest_lantern_transform, lantern) = lantern_snapshot
        .iter()
        .min_by(|(a, _), (b, _)| {
            moth_transform
                .translation
                .distance_squared(a.translation)
                .partial_cmp(&moth_transform.translation.distance_squared(b.translation))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    let direction_to_lantern =
        (closest_lantern_transform.translation - moth_transform.translation).normalize_or_zero();
    direction_to_lantern * lantern.radiance
}

pub fn moth_landing_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<FlockingConfig>,
    lanterns: Query<&Transform, With<Lantern>>,
    mut moths: Query<(Entity, &Transform, &mut Velocity, Option<&mut LandedTimer>), With<Moth>>,
) {
    let mut rng = rand::rng();
    let lantern_transforms: Vec<&Transform> = lanterns.iter().collect();

    if lantern_transforms.is_empty() {
        return;
    }

    for (moth_entity, moth_transform, mut velocity, maybe_landed_timer) in &mut moths {
        let action = match maybe_landed_timer {
            Some(mut landed_timer) => determine_takeoff_action(&mut landed_timer, &time),
            None => {
                determine_landing_action(moth_transform, &lantern_transforms, &config, &mut rng)
            }
        };

        match action {
            MothAction::TakeOff => {
                commands.entity(moth_entity).remove::<LandedTimer>();
                velocity.0 = Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-1.0..1.0),
                )
                .normalize_or_zero()
                    * config.moth_speed;
            }
            MothAction::Land => {
                velocity.0 = Vec3::ZERO;
                commands
                    .entity(moth_entity)
                    .insert(LandedTimer(Timer::from_seconds(
                        config.landed_duration_secs,
                        TimerMode::Once,
                    )));
            }
            MothAction::DoNothing => {}
        }
    }
}

#[derive(PartialEq, Debug)]
enum MothAction {
    TakeOff,
    Land,
    DoNothing,
}

fn determine_takeoff_action(landed_timer: &mut LandedTimer, time: &Time) -> MothAction {
    landed_timer.0.tick(time.delta());
    if landed_timer.0.finished() {
        MothAction::TakeOff
    } else {
        MothAction::DoNothing
    }
}

fn should_land(is_close_enough: bool, random_roll_succeeded: bool) -> MothAction {
    if is_close_enough && random_roll_succeeded {
        MothAction::Land
    } else {
        MothAction::DoNothing
    }
}

fn determine_landing_action(
    moth_transform: &Transform,
    lantern_transforms: &[&Transform],
    config: &FlockingConfig,
    rng: &mut impl Rng,
) -> MothAction {
    let closest_lantern = lantern_transforms
        .iter()
        .min_by(|a, b| {
            moth_transform
                .translation
                .distance_squared(a.translation)
                .partial_cmp(&moth_transform.translation.distance_squared(b.translation))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    let distance_to_lantern = moth_transform
        .translation
        .distance(closest_lantern.translation);

    let is_close_enough = distance_to_lantern < config.landing_distance;
    let random_roll_succeeded = rng.random_bool(config.landing_chance);

    should_land(is_close_enough, random_roll_succeeded)
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::entity::Entity;
    use bevy::time::{Timer, TimerMode};
    use std::time::Duration;

    #[test]
    fn test_determine_takeoff_action_when_timer_is_finished() {
        let mut timer = LandedTimer(Timer::from_seconds(1.0, TimerMode::Once));
        let mut time = Time::default();
        time.advance_by(Duration::from_secs(2));

        let action = determine_takeoff_action(&mut timer, &time);
        assert_eq!(action, MothAction::TakeOff);
    }

    #[test]
    fn test_determine_takeoff_action_when_timer_is_not_finished() {
        let mut timer = LandedTimer(Timer::from_seconds(2.0, TimerMode::Once));
        let mut time = Time::default();
        time.advance_by(Duration::from_secs(1));

        let action = determine_takeoff_action(&mut timer, &time);
        assert_eq!(action, MothAction::DoNothing);
    }

    #[test]
    fn test_calculate_attraction_force_no_lanterns() {
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let lantern_snapshot = vec![];
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot);
        assert_eq!(force, Vec3::ZERO);
    }

    #[test]
    fn test_calculate_attraction_force_one_lantern() {
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let lantern_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let lantern = Lantern { radiance: 1.0 };
        let lantern_snapshot = vec![(&lantern_transform, &lantern)];
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot);
        assert_eq!(force, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_calculate_attraction_force_multiple_lanterns() {
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let close_lantern_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let close_lantern = Lantern { radiance: 1.0 };
        let far_lantern_transform = Transform::from_translation(Vec3::new(-20.0, 0.0, 0.0));
        let far_lantern = Lantern { radiance: 1.0 };
        let lantern_snapshot = vec![
            (&close_lantern_transform, &close_lantern),
            (&far_lantern_transform, &far_lantern),
        ];
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot);
        assert_eq!(force, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_should_land_true_true() {
        assert_eq!(should_land(true, true), MothAction::Land);
    }

    #[test]
    fn test_should_land_true_false() {
        assert_eq!(should_land(true, false), MothAction::DoNothing);
    }

    #[test]
    fn test_should_land_false_true() {
        assert_eq!(should_land(false, true), MothAction::DoNothing);
    }

    #[test]
    fn test_should_land_false_false() {
        assert_eq!(should_land(false, false), MothAction::DoNothing);
    }

    #[test]
    fn test_calculate_flocking_forces_no_flockmates() {
        let moth_entity = Entity::from_raw(0);
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let flock_snapshot = vec![(
            moth_entity,
            moth_transform,
            Velocity(Vec3::new(1.0, 0.0, 0.0)),
        )];
        let config = FlockingConfig::default();

        let (separation, alignment, cohesion, local_flockmates) =
            calculate_flocking_forces(moth_entity, &moth_transform, &flock_snapshot, &config);

        assert_eq!(separation, Vec3::ZERO);
        assert_eq!(alignment, Vec3::ZERO);
        assert_eq!(cohesion, Vec3::ZERO);
        assert_eq!(local_flockmates, 0);
    }

    #[test]
    fn test_calculate_flocking_forces_one_flockmate() {
        let moth1_entity = Entity::from_raw(0);
        let moth1_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

        let moth2_entity = Entity::from_raw(1);
        let moth2_transform = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let moth2_velocity = Velocity(Vec3::new(1.0, 0.0, 0.0));

        let flock_snapshot = vec![
            (
                moth1_entity,
                moth1_transform,
                Velocity(Vec3::new(1.0, 0.0, 0.0)),
            ),
            (moth2_entity, moth2_transform, moth2_velocity),
        ];
        let config = FlockingConfig {
            perception_radius: 2.0,
            ..Default::default()
        };

        let (separation, alignment, cohesion, local_flockmates) =
            calculate_flocking_forces(moth1_entity, &moth1_transform, &flock_snapshot, &config);

        assert_eq!(separation, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(alignment, moth2_velocity.0);
        assert_eq!(cohesion, moth2_transform.translation);
        assert_eq!(local_flockmates, 1);
    }

    #[test]
    fn test_calculate_flocking_forces_one_flockmate_outside_perception_radius() {
        let moth1_entity = Entity::from_raw(0);
        let moth1_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

        let moth2_entity = Entity::from_raw(1);
        let moth2_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let moth2_velocity = Velocity(Vec3::new(1.0, 0.0, 0.0));

        let flock_snapshot = vec![
            (
                moth1_entity,
                moth1_transform,
                Velocity(Vec3::new(1.0, 0.0, 0.0)),
            ),
            (moth2_entity, moth2_transform, moth2_velocity),
        ];
        let config = FlockingConfig {
            perception_radius: 5.0,
            ..Default::default()
        };

        let (separation, alignment, cohesion, local_flockmates) =
            calculate_flocking_forces(moth1_entity, &moth1_transform, &flock_snapshot, &config);

        assert_eq!(separation, Vec3::ZERO);
        assert_eq!(alignment, Vec3::ZERO);
        assert_eq!(cohesion, Vec3::ZERO);
        assert_eq!(local_flockmates, 0);
    }
}
