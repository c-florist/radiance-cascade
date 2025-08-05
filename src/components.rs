use bevy::prelude::*;

#[derive(Component)]
pub struct Moth;

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Lantern {
    pub radiance: f32,
}

#[derive(Component)]
pub struct LandedTimer(pub Timer);
