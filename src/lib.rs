pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone)]
pub struct Grid {
    buffer: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let buffer: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self {
            height: buffer.len(),
            width: buffer[0].len(),
            buffer,
        }
    }

    pub fn char_at(&self, position: &Vec2) -> char {
        self.buffer[position.y as usize][position.x as usize]
    }

    pub fn replace_char_at(&mut self, position: &Vec2, new_char: char) {
        self.buffer[position.y as usize][position.x as usize] = new_char;
    }

    pub fn find_first_char_position(&self, needle: char) -> Option<Vec2> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.buffer[y][x] == needle {
                    return Some(Vec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        None
    }

    pub fn find_char_positions(&self, needle: char) -> Vec<Vec2> {
        let mut positions = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.buffer[y][x] == needle {
                    positions.push(Vec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        positions
    }

    pub fn is_point_in_gird(&self, point: &Vec2) -> bool {
        point.y >= 0 && point.y < self.height as i32 && point.x >= 0 && point.x < self.width as i32
    }

    pub fn get_chars_in_direction(
        &self,
        start_from: &Vec2,
        direction: &Direction,
        count: usize,
    ) -> Vec<char> {
        let offset = direction.get_offset();

        let mut current_position = start_from.add(&offset);
        let mut chars = vec![];
        for _ in 0..count {
            if self.is_point_in_gird(&current_position) {
                chars.push(self.buffer[current_position.y as usize][current_position.x as usize])
            }

            current_position = current_position.add(&offset);
        }

        chars
    }

    pub fn get_cell_in_direction(
        &self,
        start_from: &Vec2,
        direction: &Direction,
    ) -> Option<(Vec2, char)> {
        let chars = self.get_chars_in_direction(start_from, direction, 1);

        if chars.len() == 1 {
            return Some((start_from.add(&direction.get_offset()), chars[0]));
        }

        None
    }

    pub fn make_subgrid(&self, start_from: &Vec2, width: usize, height: usize) -> Option<Grid> {
        // one needs to be subtracted because we want the start_from point to be included in the subgrid
        let lower_right_point = start_from.add(&Vec2 {
            x: width as i32 - 1,
            y: height as i32 - 1,
        });
        if self.is_point_in_gird(&lower_right_point) {
            let mut subgrid = vec![];
            for y in start_from.y..=lower_right_point.y {
                subgrid.push(vec![]);

                for x in start_from.x..=lower_right_point.x {
                    subgrid
                        .last_mut()?
                        .push(self.buffer[y as usize][x as usize]);
                }
            }

            return Some(Grid {
                height: subgrid.len(),
                width: subgrid[0].len(),
                buffer: subgrid,
            });
        }

        None
    }

    pub fn iter(&self) -> GridIter {
        GridIter {
            grid: self,
            current_x: 0,
            current_y: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn pretty_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.buffer[y][x]);
            }
            println!();
        }

        println!();
    }
}

pub struct GridIter<'a> {
    grid: &'a Grid,
    current_x: usize,
    current_y: usize,
}

impl Iterator for GridIter<'_> {
    type Item = (Vec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.grid.height {
            return None;
        }
        let pos = Vec2 {
            x: self.current_x as i32,
            y: self.current_y as i32,
        };
        let value = self.grid.char_at(&pos);

        self.current_x += 1;
        if self.current_x >= self.grid.width {
            self.current_x = 0;
            self.current_y += 1;
        }

        Some((pos, value))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vec2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vec2) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
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
