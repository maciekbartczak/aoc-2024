advent_of_code::solution!(9);

#[derive(Debug, Clone)]
enum Block {
    File { id: u64, length: usize },
    Empty,
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn from_input(input: &str) -> Self {
        let input = input.trim();
        let mut current_id = 0;
        let mut is_file = true;

        let blocks = input
            .chars()
            .flat_map(|c| {
                let block_count = c.to_digit(10).unwrap() as usize;
                let block = if is_file {
                    let file_block = Block::File {
                        id: current_id,
                        length: block_count,
                    };
                    current_id += 1;
                    file_block
                } else {
                    Block::Empty
                };
                is_file = !is_file;

                vec![block; block_count]
            })
            .collect();

        Self { blocks }
    }

    pub fn compact(&mut self) {
        for idx in 0..self.blocks.len() {
            let block = &self.blocks[idx];
            if matches!(block, Block::Empty) {
                if let Some(file_block_idx) = self.find_rightmost_file_block_idx(idx) {
                    self.blocks.swap(idx, file_block_idx)
                } else {
                    // there are no gaps left
                    return;
                }
            }
        }
    }

    pub fn defragment(&mut self) {
        let mut idx = self.blocks.len() - 1;
        while idx != 0 {
            if let Block::File { length, .. } = self.blocks[idx] {
                let file_start_idx = idx.saturating_sub(length - 1);

                if let Some(empty_space_start_idx) =
                    self.find_empty_space_with_min_length_start_idx(length, file_start_idx)
                {
                    let mut empty_idx = empty_space_start_idx;
                    while idx >= file_start_idx {
                        self.blocks.swap(empty_idx, idx);

                        idx -= 1;
                        empty_idx += 1;
                    }
                } else {
                    // move to the next sequence of blocks if no space is found to move this file
                    idx = file_start_idx.saturating_sub(1);
                }
            } else {
                idx -= 1;
            }
        }
    }

    pub fn find_rightmost_file_block_idx(&self, left_boundary: usize) -> Option<usize> {
        for idx in (left_boundary..self.blocks.len()).rev() {
            if matches!(self.blocks[idx], Block::File { .. }) {
                return Some(idx);
            }
        }

        None
    }

    pub fn calculate_checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, block)| match block {
                Block::File { id, .. } => idx as u64 * id,
                Block::Empty => 0,
            })
            .sum()
    }

    fn find_empty_space_with_min_length_start_idx(
        &self,
        min_length: usize,
        right_boundary: usize,
    ) -> Option<usize> {
        for idx in 0..right_boundary {
            if let Block::Empty = self.blocks[idx] {
                let empty_blocks_length = self.blocks[idx..]
                    .iter()
                    .take_while(|&b| matches!(b, Block::Empty))
                    .count();

                if empty_blocks_length >= min_length {
                    return Some(idx);
                }
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);

    disk.compact();

    Some(disk.calculate_checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);

    disk.defragment();

    Some(disk.calculate_checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
