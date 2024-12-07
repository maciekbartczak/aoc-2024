use advent_of_code::{Direction, Grid, Vec2};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    Some(count_occurrences_in_grid(
        &grid,
        vec!['X', 'M', 'A', 'S'],
        false,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);

    let mut count: u32 = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let current_position = Vec2::new(x as i32, y as i32);

            if let Some(subgrid) = grid.make_subgrid(&current_position, 3, 3) {
                if count_occurrences_in_grid(&subgrid, vec!['M', 'A', 'S'], true) == 2 {
                    count += 1;
                }
            };
        }
    }

    Some(count)
}

fn count_occurrences_in_grid(grid: &Grid, needle: Vec<char>, only_diagonals: bool) -> u32 {
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
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let current_position = Vec2::new(x as i32, y as i32);
            if grid.char_at(&current_position) == wanted_first_char {
                for dir in &directions {
                    let remainder = grid.get_chars_in_direction(&current_position, dir, 3);
                    if remainder == wanted_remainder {
                        count += 1
                    }
                }
            }
        }
    }

    count
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
