use bevy::prelude::*;

#[derive(Component)]
pub struct Moth;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Lantern {
    pub radiance: f32,
}
