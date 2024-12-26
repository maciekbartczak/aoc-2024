use advent_of_code::{Direction, Grid, Vec2};
use itertools::Itertools;

advent_of_code::solution!(15);

type Move = Direction;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, moves) = parse_input(input);
    let mut robot_pos = grid.find_first_char_position('@').unwrap();

    for m in moves {
        let next_robot_pos = robot_pos.add(&m.get_offset());
        let next_cell = grid.char_at(&next_robot_pos);

        if next_cell == '.' {
            grid.swap_cells(&robot_pos, &next_robot_pos);
            robot_pos = next_robot_pos;
        }
        if next_cell == 'O' {
            if let Some(empty_cell_pos) = try_find_empty_cell(&robot_pos, &m, &grid) {
                grid.swap_cells(&next_robot_pos, &empty_cell_pos);
                grid.swap_cells(&next_robot_pos, &robot_pos);
                robot_pos = next_robot_pos;
            }
        }
    }

    Some(
        grid.find_char_positions('O')
            .iter()
            .map(|b| distance(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, moves) = parse_input(input);
    let mut grid = scale_grid(grid);
    let mut robot_pos = grid.find_first_char_position('@').unwrap();

    for m in moves.iter() {
        let next_robot_pos = robot_pos.add(&m.get_offset());
        let next_cell = grid.char_at(&next_robot_pos);

        if next_cell == '.' {
            grid.swap_cells(&robot_pos, &next_robot_pos);
            robot_pos = next_robot_pos;
        }
        if next_cell == '[' || next_cell == ']' {
            match m {
                Direction::Right => {
                    if let Some(empty_cell_pos) = try_find_empty_cell(&robot_pos, &m, &grid) {
                        for x in (robot_pos.x..empty_cell_pos.x).rev() {
                            let p = Vec2::new(x, robot_pos.y);
                            let p2 = Vec2::new(x + 1, robot_pos.y);
                            grid.swap_cells(&p, &p2);
                        }
                        robot_pos = next_robot_pos;
                    }
                }
                Direction::Left => {
                    if let Some(empty_cell_pos) = try_find_empty_cell(&robot_pos, &m, &grid) {
                        for x in empty_cell_pos.x..robot_pos.x {
                            let p = Vec2::new(x, robot_pos.y);
                            let p2 = Vec2::new(x + 1, robot_pos.y);
                            grid.swap_cells(&p, &p2);
                        }
                        robot_pos = next_robot_pos;
                    }
                }
                Direction::Up => {
                    let boxes_to_move = find_boxes_to_move_vertically(&next_robot_pos, &m, &grid);

                    let mut can_move = true;
                    for (l, r) in &boxes_to_move {
                        let l_empty_cell_pos = l.add(&Direction::Up.get_offset());
                        if grid.char_at(&l_empty_cell_pos) == '#' {
                            can_move = false;
                            break;
                        }
                        let r_empty_cell_pos = r.add(&Direction::Up.get_offset());
                        if grid.char_at(&r_empty_cell_pos) == '#' {
                            can_move = false;
                            break;
                        }
                    }

                    if can_move {
                        for (l, r) in boxes_to_move.iter().sorted_by_key(|b| b.0.y) {
                            if grid.char_at(&l.add(&Direction::Up.get_offset())) == '#' || grid.char_at(&r.add(&Direction::Up.get_offset()))== '#' {
                                panic!("moved a wall");
                            }
                            grid.swap_cells(l, &l.add(&Direction::Up.get_offset()));
                            grid.swap_cells(r, &r.add(&Direction::Up.get_offset()));
                        }

                        grid.swap_cells(&robot_pos, &next_robot_pos);
                        robot_pos = next_robot_pos;
                    }
                }
                Direction::Down => {
                    let boxes_to_move = find_boxes_to_move_vertically(&next_robot_pos, &m, &grid);

                    let mut can_move = true;
                    for (l, r) in &boxes_to_move {
                        let l_empty_cell_pos = l.add(&Direction::Down.get_offset());
                        if grid.char_at(&l_empty_cell_pos) == '#' {
                            can_move = false;
                            break;
                        }
                        let r_empty_cell_pos = r.add(&Direction::Down.get_offset());
                        if grid.char_at(&r_empty_cell_pos) == '#' {
                            can_move = false;
                            break;
                        }
                    }

                    if can_move {
                        for (l, r) in boxes_to_move.iter().sorted_by_key(|b| b.0.y).rev() {
                            if grid.char_at(&l.add(&Direction::Down.get_offset())) == '#' || grid.char_at(&r.add(&Direction::Down.get_offset()))== '#' {
                                grid.pretty_print();
                                panic!("moved a wall");
                            }
                            grid.swap_cells(l, &l.add(&Direction::Down.get_offset()));
                            grid.swap_cells(r, &r.add(&Direction::Down.get_offset()));
                        }

                        grid.swap_cells(&robot_pos, &next_robot_pos);
                        robot_pos = next_robot_pos;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Some(
        grid.find_char_positions('[')
            .iter()
            .map(|b| distance(b))
            .sum(),
    )
}

fn find_boxes_to_move_vertically(
    current_pos: &Vec2,
    dir: &Direction,
    grid: &Grid,
) -> Vec<(Vec2, Vec2)> {
    if grid.char_at(&current_pos) != '[' && grid.char_at(&current_pos) != ']' {
        return vec![];
    }

    let is_left_part_of_box = grid.char_at(&current_pos) == '[';

    let left_part_of_box_pos = if is_left_part_of_box {
        *current_pos
    } else {
        current_pos.add(&Direction::Left.get_offset())
    };
    let right_part_of_box_pos = if is_left_part_of_box {
        current_pos.add(&Direction::Right.get_offset())
    } else {
        *current_pos
    };

    let b = (left_part_of_box_pos, right_part_of_box_pos);
    let mut boxes = vec![b];

    let mut stacked_boxes_left =
        find_boxes_to_move_vertically(&left_part_of_box_pos.add(&dir.get_offset()), dir, grid);
    boxes.append(&mut stacked_boxes_left);

    let mut stacked_boxes_right =
        find_boxes_to_move_vertically(&right_part_of_box_pos.add(&dir.get_offset()), dir, grid);
    boxes.append(&mut stacked_boxes_right);

    boxes.into_iter().unique().collect()
}

fn scale_grid(grid: Grid) -> Grid {
    let width = grid.width() * 2;
    let height = grid.height();
    let mut new_grid = Grid::new(width, height);

    for y in 0..height {
        for x in (0..width).step_by(2) {
            let pos = Vec2::new(x as i32, y as i32);

            let og_c = grid.char_at(&pos.div(&Vec2::new(2, 1)));
            let (l, r) = match og_c {
                '#' => ('#', '#'),
                'O' => ('[', ']'),
                '.' => ('.', '.'),
                '@' => ('@', '.'),
                _ => unreachable!(),
            };

            new_grid.replace_char_at(&pos, l);
            new_grid.replace_char_at(&pos.add(&Direction::Right.get_offset()), r);
        }
    }

    new_grid
}

fn try_find_empty_cell(starting_pos: &Vec2, m: &Direction, grid: &Grid) -> Option<Vec2> {
    let offset = m.get_offset();
    let mut current = starting_pos.add(&offset);
    while grid.is_point_in_gird(&current) {
        if grid.char_at(&current) == '#' {
            return None;
        }
        if grid.char_at(&current) == '.' {
            return Some(current);
        }
        current = current.add(&offset);
    }

    None
}

fn distance(pos: &Vec2) -> u32 {
    (100 * pos.y + pos.x) as u32
}

fn parse_input(input: &str) -> (Grid, Vec<Move>) {
    let mut input_split = input.split("\n\n");

    let grid = Grid::from_input(input_split.next().unwrap());
    let moves = input_split
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            _ => None,
        })
        .collect();

    (grid, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
