use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;

/// Cell size should be adjusted based on the average size of entities
/// and the view radius of the moths.
pub const CELL_SIZE: f32 = 15.0;

#[derive(Resource, Default)]
pub struct SpatialIndex {
    pub map: HashMap<(i32, i32), HashSet<Entity>>,
}

impl SpatialIndex {
    pub fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        let mut nearby = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if let Some(entities) = self.map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(entities.iter());
                }
            }
        }
        nearby
    }
}
