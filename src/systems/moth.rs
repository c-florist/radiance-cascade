use crate::components::{LandedTimer, Lantern, Moth, Velocity};
use crate::config::{LanternConfig, MothConfig};
use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Debug)]
enum MothAction {
    TakeOff,
    Land,
    DoNothing,
}

pub fn moth_wander_system(
    moth_config: Res<MothConfig>,
    lantern_config: Res<LanternConfig>,
    mut queries: ParamSet<(
        Query<(&Transform, &mut Velocity), (With<Moth>, Without<LandedTimer>)>,
        Query<(&Transform, &Velocity), With<Moth>>,
    )>,
    lanterns: Query<(&Transform, &Lantern)>,
) {
    let mut rng = rand::rng();

    let active_lanterns: Vec<(&Transform, &Lantern)> = lanterns
        .iter()
        .filter(|(_, lantern)| lantern.is_on)
        .collect();

    for (transform, mut velocity) in queries.p0().iter_mut() {
        if velocity.0 == Vec3::ZERO {
            velocity.0 = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            )
            .normalize_or_zero();
        }

        let wander_force = (Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize_or_zero()
            * moth_config.wander_factor)
            .normalize_or_zero();

        let force = calculate_attraction_force(transform, &active_lanterns, &lantern_config);

        velocity.0 += force + wander_force;
        velocity.0 = velocity.0.normalize_or_zero() * moth_config.moth_speed;
    }
}

/// Given a moth's position, find the closest lantern and calculate the
/// attraction force using the lantern's radiance as scale factor.
fn calculate_attraction_force(
    moth_transform: &Transform,
    lantern_snapshot: &[(&Transform, &Lantern)],
    lantern_config: &LanternConfig,
) -> Vec3 {
    if lantern_snapshot.is_empty() {
        return Vec3::ZERO;
    }

    let Some((closest_lantern_transform, lantern)) =
        lantern_snapshot.iter().min_by(|(a, _), (b, _)| {
            moth_transform
                .translation
                .distance_squared(a.translation)
                .partial_cmp(&moth_transform.translation.distance_squared(b.translation))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    else {
        return Vec3::ZERO;
    };

    let distance = moth_transform
        .translation
        .distance(closest_lantern_transform.translation);

    if distance < lantern_config.personal_space {
        // If too close, calculate repulsion force
        let direction_away_from_lantern = (moth_transform.translation
            - closest_lantern_transform.translation)
            .normalize_or_zero();
        direction_away_from_lantern * (1.0 / distance)
    } else {
        // Otherwise, calculate attraction force
        let direction_to_lantern = (closest_lantern_transform.translation
            - moth_transform.translation)
            .normalize_or_zero();
        direction_to_lantern * lantern.radiance
    }
}

pub fn moth_landing_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<MothConfig>,
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
    config: &MothConfig,
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

pub fn moth_movement_system(
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
    use bevy::app::App;
    use bevy::time::{Timer, TimerMode};
    use std::time::Duration;

    #[test]
    fn test_moth_wander_system_gives_velocity_to_moth_with_zero_velocity() {
        let mut app = App::new();
        app.insert_resource(MothConfig::default());
        app.insert_resource(LanternConfig::default());
        app.add_systems(Update, moth_wander_system);

        let moth_entity = app
            .world_mut()
            .spawn((Moth, Transform::default(), Velocity(Vec3::ZERO)))
            .id();

        app.update();

        let moth_velocity = app.world().get::<Velocity>(moth_entity).unwrap();
        assert_ne!(moth_velocity.0, Vec3::ZERO);
    }

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
        let lantern_config = LanternConfig::default();
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot, &lantern_config);
        assert_eq!(force, Vec3::ZERO);
    }

    #[test]
    fn test_calculate_attraction_force_one_lantern() {
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let lantern_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let lantern = Lantern {
            radiance: 1.0,
            is_on: true,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            grid_pos: (1, 1),
        };
        let lantern_snapshot = vec![(&lantern_transform, &lantern)];
        let lantern_config = LanternConfig::default();
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot, &lantern_config);
        assert_eq!(force, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_calculate_attraction_force_multiple_lanterns() {
        let moth_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let close_lantern_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
        let close_lantern = Lantern {
            radiance: 1.0,
            is_on: true,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            grid_pos: (1, 1),
        };
        let far_lantern_transform = Transform::from_translation(Vec3::new(-20.0, 0.0, 0.0));
        let far_lantern = Lantern {
            radiance: 1.0,
            is_on: true,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            grid_pos: (5, 5),
        };
        let lantern_snapshot = vec![
            (&close_lantern_transform, &close_lantern),
            (&far_lantern_transform, &far_lantern),
        ];
        let lantern_config = LanternConfig::default();
        let force = calculate_attraction_force(&moth_transform, &lantern_snapshot, &lantern_config);
        // The moth should be attracted to the closest lantern.
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
    fn test_moth_avoids_lantern_personal_space() {
        let mut app = App::new();
        app.insert_resource(MothConfig::default());
        app.insert_resource(LanternConfig::default());
        app.add_systems(Update, moth_wander_system);

        app.world_mut().spawn((
            Lantern {
                radiance: 1.0,
                is_on: true,
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                grid_pos: (0, 0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        let moth_entity = app
            .world_mut()
            .spawn((
                Moth,
                Transform::from_xyz(0.1, 0.0, 0.0),
                Velocity(Vec3::new(-1.0, 0.0, 0.0)),
            ))
            .id();

        app.update();

        let moth_velocity = app.world().get::<Velocity>(moth_entity).unwrap();
        assert!(
            moth_velocity.0.x >= 0.0,
            "Moth should be pushed away from the lantern"
        );
    }
}
