use crate::components::Lantern;
use crate::resources::{CELL_SIZE, SpatialIndex};
use bevy::prelude::*;

pub fn setup_lantern_index(
    mut spatial_index: ResMut<SpatialIndex>,
    lantern_query: Query<(Entity, &Transform), With<Lantern>>,
) {
    for (entity, transform) in lantern_query.iter() {
        let tile = (
            (transform.translation.x / CELL_SIZE).floor() as i32,
            (transform.translation.y / CELL_SIZE).floor() as i32,
        );
        spatial_index.map.entry(tile).or_default().insert(entity);
    }
}
