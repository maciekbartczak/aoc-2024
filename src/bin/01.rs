use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let sum = left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);

    let mut seen_ids = HashMap::new();

    let sum = left
        .iter()
        .map(|id| {
            let count = seen_ids
                .entry(id)
                .or_insert_with(|| count_occurences(id, &right));
            *id * *count
        })
        .sum();

    Some(sum)
}

fn count_occurences(id: &u32, list: &Vec<u32>) -> u32 {
    list.iter().filter(|value| **value == *id).count() as u32
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines();

    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in lines {
        let mut split = line.split("   ");
        left_list.push(split.next().unwrap().parse::<u32>().unwrap());
        right_list.push(split.next().unwrap().parse::<u32>().unwrap());
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
