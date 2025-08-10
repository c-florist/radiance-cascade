use crate::components::{Lantern, Moth, Velocity};
use crate::config::MothConfig;
use bevy::prelude::*;

pub fn moth_attraction_system(
    moth_config: Res<MothConfig>,
    mut moth_query: Query<(&Transform, &mut Velocity), With<Moth>>,
    lantern_query: Query<(&Transform, &Lantern)>,
) {
    let active_lanterns: Vec<_> = lantern_query
        .iter()
        .filter(|(_, lantern)| lantern.is_on)
        .collect();

    if active_lanterns.is_empty() {
        return;
    }

    for (moth_transform, mut velocity) in moth_query.iter_mut() {
        let best_lantern = active_lanterns
            .iter()
            .filter(|(lantern_transform, _)| {
                moth_transform
                    .translation
                    .distance_squared(lantern_transform.translation)
                    < moth_config.view_radius.powi(2)
            })
            .max_by(|(a_transform, a_lantern), (b_transform, b_lantern)| {
                let a_radiance = a_lantern.radiance;
                let b_radiance = b_lantern.radiance;

                if (a_radiance - b_radiance).abs() > f32::EPSILON {
                    a_radiance.partial_cmp(&b_radiance).unwrap()
                } else {
                    let a_dist = moth_transform
                        .translation
                        .distance_squared(a_transform.translation);
                    let b_dist = moth_transform
                        .translation
                        .distance_squared(b_transform.translation);
                    b_dist.partial_cmp(&a_dist).unwrap()
                }
            });

        if let Some(best_lantern) = best_lantern {
            let direction =
                (best_lantern.0.translation - moth_transform.translation).normalize_or_zero();
            velocity.0 = direction * moth_config.moth_speed;
        }
    }
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

    #[test]
    fn test_moth_moves_towards_closest_lantern() {
        // Setup
        let mut app = App::new();
        app.insert_resource(MothConfig::default());
        app.add_systems(Update, moth_attraction_system);

        // Arrange
        let moth_id = app
            .world_mut()
            .spawn((
                Moth,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Velocity(Vec3::ZERO),
            ))
            .id();

        // Far lantern
        app.world_mut().spawn((
            Lantern {
                is_on: true,
                ..Default::default()
            },
            Transform::from_xyz(100.0, 0.0, 0.0),
        ));

        // Close lantern
        app.world_mut().spawn((
            Lantern {
                is_on: true,
                ..Default::default()
            },
            Transform::from_xyz(10.0, 0.0, 0.0),
        ));

        // Off lantern (should be ignored)
        app.world_mut().spawn((
            Lantern {
                is_on: false,
                ..Default::default()
            },
            Transform::from_xyz(1.0, 0.0, 0.0),
        ));

        // Act
        app.update();

        // Assert
        let moth_velocity = app.world().get::<Velocity>(moth_id).unwrap();
        assert_eq!(
            moth_velocity.0.normalize_or_zero(),
            Vec3::new(1.0, 0.0, 0.0),
            "Moth should move towards the closest 'on' lantern"
        );
    }

    #[test]
    fn test_moth_is_attracted_to_lanterns_within_view_radius() {
        let mut app = App::new();
        app.insert_resource(MothConfig::default());
        app.add_systems(Update, moth_attraction_system);

        let moth_id = app
            .world_mut()
            .spawn((
                Moth,
                Transform::from_xyz(0.0, 0.0, 0.0),
                Velocity(Vec3::ZERO),
            ))
            .id();

        // Lantern A: Inside view radius, low radiance
        app.world_mut().spawn((
            Lantern {
                is_on: true,
                radiance: 10.0,
                ..Default::default()
            },
            Transform::from_xyz(10.0, 0.0, 0.0),
        ));

        // Lantern B: Inside view radius, high radiance
        app.world_mut().spawn((
            Lantern {
                is_on: true,
                radiance: 20.0,
                ..Default::default()
            },
            Transform::from_xyz(-12.0, 0.0, 0.0),
        ));

        // Lantern C: Outside view radius, highest radiance
        app.world_mut().spawn((
            Lantern {
                is_on: true,
                radiance: 100.0,
                ..Default::default()
            },
            Transform::from_xyz(30.0, 0.0, 0.0),
        ));

        app.update();

        let moth_velocity = app.world().get::<Velocity>(moth_id).unwrap();
        assert_eq!(
            moth_velocity.0.normalize_or_zero(),
            Vec3::new(-1.0, 0.0, 0.0),
            "Moth should move towards the most radiant lantern within its view radius"
        );
    }
}
