use bevy::{
    input::touch::{TouchInput, TouchPhase},
    prelude::*,
};

use crate::components::{Moth, OrbitCamera, Velocity};
use crate::resources::TouchState;

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

pub fn camera_control_system(
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera), With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut touch_events: EventReader<TouchInput>,
    mut touch_state: Local<TouchState>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut orbit_camera)) = camera_query.single_mut() {
        let mut angle_delta = 0.0;

        // Keyboard controls
        let keyboard_speed = 1.5;
        if keys.pressed(KeyCode::ArrowLeft) {
            angle_delta -= keyboard_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowRight) {
            angle_delta += keyboard_speed * time.delta_secs();
        }

        // Touch controls
        let touch_speed = 0.1;
        for ev in touch_events.read() {
            match ev.phase {
                TouchPhase::Started => {
                    touch_state.start_pos = Some(ev.position);
                    touch_state.last_pos = Some(ev.position);
                }
                TouchPhase::Moved => {
                    if let Some(last_pos) = touch_state.last_pos {
                        let delta = ev.position - last_pos;
                        angle_delta += delta.x * touch_speed * time.delta_secs();
                    }
                    touch_state.last_pos = Some(ev.position);
                }
                TouchPhase::Ended | TouchPhase::Canceled => {
                    touch_state.start_pos = None;
                    touch_state.last_pos = None;
                }
            }
        }

        if angle_delta.abs() > f32::EPSILON {
            orbit_camera.angle += angle_delta;

            let new_x = orbit_camera.angle.cos() * orbit_camera.radius;
            let new_z = orbit_camera.angle.sin() * orbit_camera.radius;

            transform.translation.x = new_x;
            transform.translation.z = new_z;

            transform.look_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        }
    }
}
