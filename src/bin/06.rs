use advent_of_code::{Direction, Grid, Vec2};
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Guard {
    position: Vec2,
    direction: Direction,
    visited_positions: HashSet<(Vec2, Direction)>,
}

impl Guard {
    fn simulate(&mut self, grid: &Grid) {
        self.visited_positions
            .insert((self.position, self.direction));

        while let Some(pos) = self.run_simulation_step(grid) {
            self.visited_positions.insert((pos, self.direction));
        }
    }

    fn find_number_of_new_obstacle_positions_that_cause_a_loop(&mut self, grid: &Grid) -> u32 {
        let mut placed_obstacles = HashSet::new();

        loop {
            // place an obstacle in front of the guard if it would not be outside the grid
            if let Some(obstacle_pos) = self.get_next_position(grid) {
                if !placed_obstacles.contains(&obstacle_pos) && grid.char_at(&obstacle_pos) == '.' {
                    let mut grid_with_obstacle = grid.clone();
                    grid_with_obstacle.replace_char_at(&obstacle_pos, '#');

                    // check if that causes a loop
                    let guard_starting_position =
                        grid_with_obstacle.find_first_char_position('^').unwrap();
                    let mut new_guard = Guard {
                        position: guard_starting_position,
                        direction: Direction::Up,
                        visited_positions: HashSet::new(),
                    };

                    let is_in_a_loop = new_guard.detect_loop(&grid_with_obstacle);
                    if is_in_a_loop {
                        placed_obstacles.insert(obstacle_pos);
                    }
                }
            }

            // move the guard to the next position
            let position = self.run_simulation_step(grid);
            if position.is_none() {
                break;
            }
        }

        placed_obstacles.len() as u32
    }

    fn detect_loop(&mut self, grid: &Grid) -> bool {
        self.visited_positions
            .insert((self.position, self.direction));

        loop {
            let position = self.run_simulation_step(grid);
            if position.is_none() {
                return false;
            }
            let pos_with_dir = (position.unwrap(), self.direction);

            if self.visited_positions.contains(&pos_with_dir) {
                return true;
            }

            self.visited_positions.insert(pos_with_dir);
        }
    }

    fn run_simulation_step(&mut self, grid: &Grid) -> Option<Vec2> {
        let next_position = match self.get_next_position(grid) {
            Some(pos) => pos,
            // out of grid
            None => return None,
        };

        match grid.char_at(&next_position) {
            '.' | '^' => {
                self.position = next_position;
            }
            '#' => {
                self.turn_right();
            }
            c => panic!("Unexpected char in grid: {}", c),
        };

        Some(self.position)
    }

    fn get_next_position(&self, grid: &Grid) -> Option<Vec2> {
        let new_position = self.position.add(&self.direction.get_offset());
        if grid.point_is_in_gird(&new_position) {
            Some(new_position)
        } else {
            None
        }
    }

    fn turn_right(&mut self) {
        self.direction = match &self.direction {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            dir => panic!("Invalid direction state: {:?}", dir),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let guard_starting_position = grid.find_first_char_position('^').unwrap();
    let mut guard = Guard {
        position: guard_starting_position,
        direction: Direction::Up,
        visited_positions: HashSet::new(),
    };

    guard.simulate(&grid);
    let unique_visited_positions: HashSet<Vec2> =
        guard.visited_positions.iter().map(|p| p.0).collect();
    Some(unique_visited_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let guard_starting_position = grid.find_first_char_position('^').unwrap();
    let mut guard = Guard {
        position: guard_starting_position,
        direction: Direction::Up,
        visited_positions: HashSet::new(),
    };

    Some(guard.find_number_of_new_obstacle_positions_that_cause_a_loop(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
