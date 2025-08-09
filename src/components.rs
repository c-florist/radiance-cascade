use bevy::prelude::*;

#[derive(Component)]
pub struct Moth;

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Lantern {
    pub radiance: f32,
    pub is_on: bool,
    pub timer: Timer,
    pub grid_pos: (i32, i32),
}

#[derive(Component)]
pub struct LandedTimer(pub Timer);

#[derive(Component)]
pub struct Wall;
