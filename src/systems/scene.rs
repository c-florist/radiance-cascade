use bevy::prelude::*;

use crate::components::{Moth, Velocity};

pub fn enforce_boundary_system(mut moth_query: Query<(&mut Transform, &mut Velocity), With<Moth>>) {
    const ROOM_RADIUS: f32 = 10.0;
    const CEILING_HEIGHT: f32 = 10.0;
    const FLOOR_HEIGHT: f32 = 0.0;

    for (mut transform, mut velocity) in moth_query.iter_mut() {
        let pos = &mut transform.translation;

        // Enforce cylindrical boundary
        let horizontal_pos = Vec2::new(pos.x, pos.z);
        if horizontal_pos.length() > ROOM_RADIUS {
            let normal = horizontal_pos.normalize() * -1.0;
            let vel_dir = Vec2::new(velocity.0.x, velocity.0.z).normalize();
            if vel_dir.dot(normal) < 0.0 {
                let reflect = vel_dir - 2.0 * vel_dir.dot(normal) * normal;
                velocity.0.x = reflect.x * velocity.0.length();
                velocity.0.z = reflect.y * velocity.0.length();
            }
            pos.x = pos.x.clamp(-ROOM_RADIUS, ROOM_RADIUS);
            pos.z = pos.z.clamp(-ROOM_RADIUS, ROOM_RADIUS);
        }

        // Enforce vertical boundary
        if pos.y > CEILING_HEIGHT {
            pos.y = CEILING_HEIGHT;
            velocity.0.y = -velocity.0.y.abs();
        } else if pos.y < FLOOR_HEIGHT {
            pos.y = FLOOR_HEIGHT;
            velocity.0.y = velocity.0.y.abs();
        }
    }
}

pub fn camera_orbit_system(mut camera_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let orbit_radius = 15.0;
        let orbit_speed = 0.08;
        let angle = time.elapsed_secs() * orbit_speed;

        camera_transform.translation =
            Vec3::new(angle.cos() * orbit_radius, 5.0, angle.sin() * orbit_radius);
        camera_transform.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
    }
}
