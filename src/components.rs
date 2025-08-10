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

impl Default for Lantern {
    fn default() -> Self {
        Self {
            radiance: 1.0,
            is_on: false,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            grid_pos: (0, 0),
        }
    }
}

#[derive(Component)]
pub struct Wall;
