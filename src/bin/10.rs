use advent_of_code::{Direction, Grid, Vec2};
use std::collections::HashSet;

advent_of_code::solution!(10);

type Path = Vec<Vec2>;
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let trailheads = grid.find_char_positions('0');
    let paths_count = trailheads
        .iter()
        .map(|head| make_paths(head, &grid, false).iter().count() as u32)
        .sum();

    Some(paths_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let trailheads = grid.find_char_positions('0');
    let paths_count = trailheads
        .iter()
        .map(|head| make_paths(head, &grid, true).iter().count() as u32)
        .sum();

    Some(paths_count)
}

fn make_paths(head: &Vec2, grid: &Grid, find_all_paths: bool) -> Vec<Path> {
    let directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    let mut paths = vec![];

    let mut path = vec![];
    let mut queue = vec![];
    let mut visited_positions: HashSet<Vec2> = HashSet::new();
    visited_positions.insert(*head);

    path.push(*head);
    queue.push(path);

    while !queue.is_empty() {
        path = queue.pop().unwrap();
        let current_position = path.last().unwrap();

        if grid.char_at(current_position) == '9' {
            paths.push(path.clone());
        }

        for direction in directions {
            let next_position = current_position.add(&direction.get_offset());
            if !visited_positions.contains(&next_position) && grid.is_point_in_gird(&next_position)
            {
                let current_height = grid.char_at(&current_position).to_digit(10).unwrap() as i32;
                let next_height = grid.char_at(&next_position).to_digit(10).unwrap() as i32;

                // only consider positions with difference of 1 in height
                if next_height - current_height == 1 {
                    if !find_all_paths {
                        visited_positions.insert(next_position);
                    }
                    let mut new_path = path.clone();
                    new_path.push(next_position);

                    queue.push(new_path);
                }
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
