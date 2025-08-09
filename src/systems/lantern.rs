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
