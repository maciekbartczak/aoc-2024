use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Stones {
    stones: Vec<u64>,
    stone_cache: HashMap<(u64, usize), u64>,
}

impl Stones {
    pub fn from_input(input: &str) -> Self {
        Self {
            stones: input
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
            stone_cache: HashMap::new(),
        }
    }

    pub fn simulate(mut self, iterations: usize) -> u64 {
        self.stones
            .clone()
            .into_iter()
            .map(|s| self.blink(s, iterations))
            .sum()
    }

    // The idea is not to store the stones but rather check how many stones each starting stone produces after n iterations.
    // To further speed things up memoization is used
    fn blink(&mut self, stone: u64, iterations: usize) -> u64 {
        if iterations == 0 {
            return 1;
        }

        let parameters = (stone, iterations);
        if self.stone_cache.contains_key(&parameters) {
            return *self.stone_cache.get(&parameters).unwrap();
        }

        let count = if stone == 0 {
            self.blink(1, iterations - 1)
        } else if has_even_number_of_digits(stone) {
            let stone_string = stone.to_string();
            let left = stone_string[..stone_string.len() / 2]
                .parse::<u64>()
                .unwrap();
            let right = stone_string[stone_string.len() / 2..]
                .parse::<u64>()
                .unwrap();
            self.blink(left, iterations - 1) + self.blink(right, iterations - 1)
        } else {
            self.blink(stone * 2024, iterations - 1)
        };

        self.stone_cache.insert(parameters, count);

        count
    }
}

fn has_even_number_of_digits(value: u64) -> bool {
    value.to_string().len() % 2 == 0
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = Stones::from_input(input);
    Some(stones.simulate(25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = Stones::from_input(input);
    Some(stones.simulate(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
