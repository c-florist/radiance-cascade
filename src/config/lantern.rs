use bevy::prelude::Resource;

#[derive(Resource)]
pub struct LanternConfig {
    pub physical_radius: f32,
    pub on_chance: f64,
    pub flicker_chance: f64,
    pub emissive_multiplier: f32,
}

impl Default for LanternConfig {
    fn default() -> Self {
        Self {
            physical_radius: 1.0,
            on_chance: 0.01,
            flicker_chance: 0.01,
            emissive_multiplier: 100.0,
        }
    }
}
