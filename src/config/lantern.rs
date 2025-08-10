use bevy::prelude::Resource;

#[derive(Resource)]
pub struct LanternConfig {
    pub physical_radius: f32,
}

impl Default for LanternConfig {
    fn default() -> Self {
        Self {
            physical_radius: 1.0,
        }
    }
}
