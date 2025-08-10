use bevy::prelude::*;

use crate::components::{Moth, Velocity};

pub fn enforce_boundary_system(mut moth_query: Query<(&mut Transform, &mut Velocity), With<Moth>>) {
    const WALL_SIZE: f32 = 10.0;

    for (mut transform, mut velocity) in moth_query.iter_mut() {
        if transform.translation.x.abs() > WALL_SIZE {
            velocity.0.x = -velocity.0.x;
            transform.translation.x = transform.translation.x.clamp(-WALL_SIZE, WALL_SIZE);
        }
        if transform.translation.y.abs() > WALL_SIZE {
            velocity.0.y = -velocity.0.y;
            transform.translation.y = transform.translation.y.clamp(-WALL_SIZE, WALL_SIZE);
        }
    }
}
