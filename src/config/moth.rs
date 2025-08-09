use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MothConfig {
    pub moth_count: usize,
    pub moth_speed: f32,

    pub attraction_weight: f32,
    pub wander_factor: f32,

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
            attraction_weight: 0.05,
            wander_factor: 0.1,
            landing_distance: 0.6,
            landing_chance: 0.08,
            landed_duration_secs: 3.0,
        }
    }
}
