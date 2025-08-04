use bevy::prelude::*;

#[derive(Component)]
pub struct Moth;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec3);
