use bevy::prelude::Resource;

/// A resource to hold all flocking-related configuration values.
#[derive(Resource)]
pub struct FlockingConfig {
    // --- Moths ---
    pub moth_count: usize,
    pub moth_speed: f32,

    // --- Flocking ---
    pub perception_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub attraction_weight: f32,

    // --- Landing ---
    pub landing_distance: f32,
    pub landing_chance: f64,
    pub landed_duration_secs: f32,
}

impl Default for FlockingConfig {
    fn default() -> Self {
        Self {
            moth_count: 150,
            moth_speed: 2.5,
            perception_radius: 1.0,
            separation_weight: 0.3,
            alignment_weight: 0.15,
            cohesion_weight: 0.1,
            attraction_weight: 0.05,
            landing_distance: 0.6,
            landing_chance: 0.05,
            landed_duration_secs: 2.5,
        }
    }
}
