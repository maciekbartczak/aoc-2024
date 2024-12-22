use std::{collections::HashSet, vec};
use advent_of_code::{Direction, Grid, Vec2};

advent_of_code::solution!(12);

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug)]
struct Region {
    id: char,
    members: Vec<Vec2>,
}

impl Region {
    pub fn area(&self) -> u32 {
        self.members.len() as u32
    }

    pub fn perimiter(&self) -> u32 {
        let mut result = 0;
        for member in &self.members {
            for direction in DIRECTIONS {
                let neighbour_cell = member.add(&direction.get_offset());
                if !self.members.contains(&neighbour_cell) {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn sides(&self) -> u32 {
        let mut fences = vec![];
        for member in &self.members {
            for direction in DIRECTIONS {
                let neighbour_cell = member.add(&direction.get_offset());
                if !self.members.contains(&neighbour_cell) {
                    fences.push((neighbour_cell, direction));
                }
            }
        }

        for (possible_side_starting_fence_pos, fence_direction) in fences.clone() {
            let offset_x = Vec2::new(1, 0);
            let mut expected_next_fence = (possible_side_starting_fence_pos.add(&offset_x), fence_direction.clone());

            while fences.contains(&expected_next_fence) {
                let index = fences.iter().position(|p| *p == expected_next_fence).unwrap();
                fences.remove(index);

                expected_next_fence.0 = expected_next_fence.0.add(&offset_x);
            }

            let offset_y = Vec2::new(0, 1);
            let mut expected_next_fence = (possible_side_starting_fence_pos.add(&offset_y), fence_direction.clone());

            while fences.contains(&expected_next_fence) {
                let index = fences.iter().position(|p| *p == expected_next_fence).unwrap();
                fences.remove(index);

                expected_next_fence.0 = expected_next_fence.0.add(&offset_y);
            }
        }

        fences.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let regions = parse_regions(&grid);

    Some(regions.iter().map(|r| r.area() * r.perimiter()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let regions = parse_regions(&grid);

    Some(regions.iter().map(|r| r.area() * r.sides()).sum())
}

fn parse_regions(grid: &Grid) -> Vec<Region> {
    let mut visited_positions: HashSet<Vec2> = HashSet::new();
    let mut regions = vec![];

    for (positon, c) in grid.iter() {
        if !visited_positions.contains(&positon) {
            let mut queue = vec![];
            let mut region = Region {
                id: c,
                members: vec![],
            };

            queue.push(positon);

            while !queue.is_empty() {
                let current_position = queue.pop().unwrap();
                if visited_positions.contains(&current_position) {
                    continue;
                }

                region.members.push(current_position);
                visited_positions.insert(current_position);

                for direction in &DIRECTIONS {
                    if let Some((neighbour_pos, neighbour_id)) =
                        grid.get_cell_in_direction(&current_position, direction)
                    {
                        if neighbour_id != region.id {
                            continue;
                        }

                        queue.push(neighbour_pos);
                    }
                }
            }

            regions.push(region);
        }
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
