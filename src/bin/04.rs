advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    Some(count_occurrences_in_grid(
        &grid,
        vec!['X', 'M', 'A', 'S'],
        false,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    let mut count: u32 = 0;
    for y in 0..height {
        for x in 0..width {
            let current_position = Vec2 {
                x: x as i32,
                y: y as i32,
            };

            if let Some(subgrid) = make_subgrid(&grid, &current_position, 3, 3) {
                if count_occurrences_in_grid(&subgrid, vec!['M', 'A', 'S'], true) == 2 {
                    count += 1;
                }
            };
        }
    }

    Some(count)
}

fn count_occurrences_in_grid(
    grid: &Vec<Vec<char>>,
    needle: Vec<char>,
    only_diagonals: bool,
) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let directions = if only_diagonals {
        vec![
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    } else {
        vec![
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    };

    let wanted_first_char = needle[0];
    let wanted_remainder = &needle[1..];

    let mut count: u32 = 0;
    for y in 0..height {
        for x in 0..width {
            let current_position = Vec2 {
                x: x as i32,
                y: y as i32,
            };
            if grid[y][x] == wanted_first_char {
                for dir in &directions {
                    let remainder = get_chars_in_direction(&grid, &current_position, dir, 3);
                    if remainder == wanted_remainder {
                        count += 1
                    }
                }
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn add(&self, other: &Vec2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    pub fn get_offset(&self) -> Vec2 {
        match self {
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::DownRight => Vec2 { x: 1, y: 1 },
            Direction::DownLeft => Vec2 { x: -1, y: 1 },
            Direction::UpRight => Vec2 { x: 1, y: -1 },
            Direction::UpLeft => Vec2 { x: -1, y: -1 },
        }
    }
}

fn get_chars_in_direction(
    grid: &Vec<Vec<char>>,
    start_from: &Vec2,
    direction: &Direction,
    count: usize,
) -> Vec<char> {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let offset = direction.get_offset();

    let mut current_position = start_from.add(&offset);
    let mut chars = vec![];
    for _ in 0..count {
        if current_position.y >= 0
            && current_position.y < height
            && current_position.x >= 0
            && current_position.x < width
        {
            chars.push(grid[current_position.y as usize][current_position.x as usize])
        }

        current_position = current_position.add(&offset);
    }

    chars
}

fn make_subgrid(
    grid: &Vec<Vec<char>>,
    start_from: &Vec2,
    width: usize,
    height: usize,
) -> Option<Vec<Vec<char>>> {
    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    let lower_right_point = start_from.add(&Vec2 {
        x: width as i32,
        y: height as i32,
    });
    if lower_right_point.y >= 0
        && lower_right_point.y <= grid_height
        && lower_right_point.x >= 0
        && lower_right_point.x <= grid_width
    {
        let mut subgrid = vec![];
        for y in start_from.y..lower_right_point.y {
            subgrid.push(vec![]);
            for x in start_from.x..lower_right_point.x {
                subgrid.last_mut()?.push(grid[y as usize][x as usize]);
            }
        }

        return Some(subgrid);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
