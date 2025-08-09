use bevy::prelude::Resource;

#[derive(Resource)]
pub struct LanternConfig {
    pub personal_space: f32,
}

impl Default for LanternConfig {
    fn default() -> Self {
        Self {
            personal_space: 1.5,
        }
    }
}
