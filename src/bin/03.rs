use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_instruction_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = vec![];
    for (_, [lhs, rhs]) in mul_instruction_re.captures_iter(input).map(|c| c.extract()) {
        results.push(lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap());
    }

    Some(results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_instruction_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_instruction_re = Regex::new(r"(do\(\))").unwrap();
    let dont_instruction_re = Regex::new(r"(don't\(\))").unwrap();

    let mut do_instruction_indices = vec![0];
    for capture in do_instruction_re.captures_iter(input) {
        do_instruction_indices.push(capture.get(0).unwrap().start())
    }

    let mut dont_instruction_indices = vec![];
    for capture in dont_instruction_re.captures_iter(input) {
        dont_instruction_indices.push(capture.get(0).unwrap().start())
    }

    let mut results = vec![];
    for capture in mul_instruction_re.captures_iter(input) {
        let position = capture.get(0).unwrap().start();
        let (_, [lhs, rhs]) = capture.extract();
        results.push((
            position,
            lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap(),
        ));
    }

    let input_len = input.len();
    let mut do_ranges = vec![];
    for do_idx in do_instruction_indices {
        let closest_dont_index =
            find_next_dont_index(&do_idx, &dont_instruction_indices).unwrap_or(input_len);
        do_ranges.push(do_idx..closest_dont_index);
    }

    let sum = results
        .iter()
        .filter(|(start, _)| {
            for do_range in &do_ranges {
                if do_range.contains(start) {
                    return true;
                }
            }

            false
        })
        .map(|(_, value)| value)
        .sum();

    Some(sum)
}

fn find_next_dont_index(do_idx: &usize, dont_instruction_indices: &Vec<usize>) -> Option<usize> {
    for idx in dont_instruction_indices {
        if idx > do_idx {
            return Some(*idx);
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
