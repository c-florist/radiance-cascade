use crate::components::Lantern;
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

fn can_turn_on(pos: (i32, i32), grid: &[((i32, i32), bool)]) -> bool {
    let neighbors = [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ];
    !grid
        .iter()
        .any(|(grid_pos, is_on)| *is_on && neighbors.contains(grid_pos))
}

pub fn lantern_power_system(
    mut lantern_query: Query<
        (
            &Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut PointLight,
            &mut Lantern,
        ),
        With<Lantern>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut rng: GlobalEntropy<WyRand>,
) {
    const TURN_ON_CHANCE: f64 = 0.01;

    let mut grid_state: Vec<((i32, i32), bool)> = lantern_query
        .iter()
        .map(|(_, _, _, lantern)| (lantern.grid_pos, lantern.is_on))
        .collect();

    for (_, mut material, mut light, mut lantern) in lantern_query.iter_mut() {
        if lantern.is_on {
            lantern.on_timer.tick(time.delta());
            if lantern.on_timer.finished() {
                lantern.is_on = false;
                lantern.cooldown.reset();
                light.intensity = 0.0;
                if let Some(material) = materials.get_mut(&mut material.0) {
                    material.emissive = Color::BLACK.to_linear();
                }
            }
        } else {
            lantern.cooldown.tick(time.delta());
            if lantern.cooldown.finished() && rng.random_bool(TURN_ON_CHANCE) {
                if can_turn_on(lantern.grid_pos, &grid_state) {
                    lantern.is_on = true;
                    lantern.on_timer.reset();
                    lantern.radiance = rng.random_range(5.0..=15.0);
                    light.intensity = rng.random_range(1000.0..8000.0);
                    if let Some(material) = materials.get_mut(&mut material.0) {
                        material.emissive = Color::srgb(1.0, 0.5, 0.0).to_linear() * 100.0;
                    }
                    if let Some(grid_lantern) = grid_state
                        .iter_mut()
                        .find(|(pos, _)| *pos == lantern.grid_pos)
                    {
                        grid_lantern.1 = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_turn_on_returns_false_when_neighbour_is_on() {
        let grid_pos = (0, 0);
        let grid_state = vec![((0, 1), true)];
        assert!(!can_turn_on(grid_pos, &grid_state));
    }

    #[test]
    fn test_can_turn_on_returns_true_when_neighbours_are_off() {
        let grid_pos = (0, 0);
        let grid_state = vec![((0, 1), false)];
        assert!(can_turn_on(grid_pos, &grid_state));
    }

    #[test]
    fn test_can_turn_on_returns_true_when_no_neighbours_exist() {
        let grid_pos = (0, 0);
        let grid_state = vec![((5, 5), true)];
        assert!(can_turn_on(grid_pos, &grid_state));
    }

    #[test]
    fn test_can_turn_on_returns_true_for_empty_grid() {
        let grid_pos = (0, 0);
        let grid_state = vec![];
        assert!(can_turn_on(grid_pos, &grid_state));
    }
}
