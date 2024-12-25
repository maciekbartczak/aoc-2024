use std::collections::HashSet;

use advent_of_code::{Direction, Vec2};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    current_pos: Vec2,
    velocity: Vec2,
}

impl Robot {
    pub fn from_line(line: &str) -> Self {
        let numbers: Vec<_> = line
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|w| !w.is_empty())
            .map(|w| w.parse::<i32>().unwrap())
            .collect();

        assert_eq!(4, numbers.len(), "Invalid input line");
        let starting_pos = Vec2::new(numbers[0], numbers[1]);
        let velocity = Vec2::new(numbers[2], numbers[3]);

        Robot {
            current_pos: starting_pos,
            velocity,
        }
    }

    pub fn simulate_move(&mut self, grid_size: &Vec2) {
        let x_offset = if self.velocity.x < 0 {
            Direction::Left.get_offset()
        } else {
            Direction::Right.get_offset()
        };

        for _ in 0..self.velocity.x.abs() {
            self.current_pos = self.current_pos.add(&x_offset);
            if self.current_pos.x >= grid_size.x {
                self.current_pos.x = 0;
            }
            if self.current_pos.x < 0 {
                self.current_pos.x = grid_size.x - 1;
            }
        }

        let y_offset = if self.velocity.y < 0 {
            Direction::Up.get_offset()
        } else {
            Direction::Down.get_offset()
        };

        for _ in 0..self.velocity.y.abs() {
            self.current_pos = self.current_pos.add(&y_offset);
            if self.current_pos.y >= grid_size.y {
                self.current_pos.y = 0;
            }
            if self.current_pos.y < 0 {
                self.current_pos.y = grid_size.y - 1;
            }
        }
    }

    fn is_in_quadrant(&self, quadrant_top_left: &Vec2, quadrant_bottom_right: &Vec2) -> bool {
        self.current_pos.gte(quadrant_top_left) && self.current_pos.lt(quadrant_bottom_right)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(|l| Robot::from_line(l)).collect();

    let grid_size = Vec2::new(101, 103);
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| r.simulate_move(&grid_size));
    }

    let mut safety_factor = 1;
    let quadrant_size = Vec2::new((grid_size.x - 1) / 2, (grid_size.y - 1) / 2);
    for x in 0..=1 {
        for y in 0..=1 {
            let mut quadrant_top_left = quadrant_size.mul(&Vec2::new(x, y));
            if x == 1 {
                quadrant_top_left.x += 1;
            }
            if y == 1 {
                quadrant_top_left.y += 1;
            }
            let quadrant_bottom_right = quadrant_top_left.add(&quadrant_size);

            let mut robot_count = 0;
            for robot in &robots {
                if robot.is_in_quadrant(&quadrant_top_left, &quadrant_bottom_right) {
                    robot_count += 1;
                }
            }

            safety_factor *= robot_count;
        }
    }

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(|l| Robot::from_line(l)).collect();

    let grid_size = Vec2::new(101, 103);
    for i in 0..10000 {
        robots.iter_mut().for_each(|r| r.simulate_move(&grid_size));

        // idk why it works, but it turns out that all positions need to be unique for the robots to create the xmas tree.
        // i guess a better way would be to calculate entropy or stdev
        let robot_positions: HashSet<Vec2> =  robots.iter().map(|r| r.current_pos).collect();
        if robot_positions.len() == robots.len() {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
