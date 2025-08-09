use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MothConfig {
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

impl Default for MothConfig {
    fn default() -> Self {
        Self {
            moth_count: 150,
            moth_speed: 1.0,
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
