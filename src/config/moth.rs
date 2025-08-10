use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MothConfig {
    pub moth_count: i32,
    pub moth_speed: f32,
}

impl Default for MothConfig {
    fn default() -> Self {
        Self {
            moth_count: 150,
            moth_speed: 1.0,
        }
    }
}
