use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MothConfig {
    pub moth_count: i32,
    pub moth_speed: f32,
    pub view_radius: f32,
    pub attraction_factor: f32,
}

impl Default for MothConfig {
    fn default() -> Self {
        Self {
            moth_count: 150,
            moth_speed: 1.0,
            view_radius: 15.0,
            attraction_factor: 0.8,
        }
    }
}
