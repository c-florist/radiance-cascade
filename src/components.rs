use bevy::prelude::*;

#[derive(Component)]
pub struct Moth;

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Lantern {
    pub radiance: f32,
    pub is_on: bool,
    pub on_timer: Timer,
    pub cooldown: Timer,
    pub grid_pos: (i32, i32),
}

impl Default for Lantern {
    fn default() -> Self {
        Self {
            radiance: 1.0,
            is_on: false,
            on_timer: Timer::from_seconds(1.0, TimerMode::Once),
            cooldown: Timer::from_seconds(10.0, TimerMode::Once),
            grid_pos: (0, 0),
        }
    }
}

#[derive(Component)]
pub struct Ceiling;
