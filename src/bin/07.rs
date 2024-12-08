advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    // solution idea:
    // https://www.reddit.com/r/adventofcode/comments/1h8rzsp/comment/m0wcuvu/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    fn is_valid(&self, extended_operator_set: bool) -> bool {
        // base case -> if there is only one element left on rhs it must equal the lhs
        let last_element = self.rhs.last().unwrap();
        if self.rhs.len() == 1 {
            return if *last_element == self.lhs {
                true
            } else {
                false
            };
        };

        if extended_operator_set {
            // Check what happens if the last operation is concatenation
            let lhs_string = self.lhs.to_string();
            let last_element_string = last_element.to_string();
            let can_be_concatenated = lhs_string.ends_with(&last_element_string);
            if can_be_concatenated {
                let mut new_rhs = self.rhs.clone();
                new_rhs.pop();

                let new_equation = Self {
                    lhs: lhs_string
                        .strip_suffix(&last_element_string)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap_or(0),
                    rhs: new_rhs,
                };

                if new_equation.is_valid(true) {
                    return true;
                }
            }
        }

        // Check what happens if the last operation is division
        let is_lhs_divisible_by_last_element = self.lhs % last_element == 0;
        if is_lhs_divisible_by_last_element {
            let mut new_rhs = self.rhs.clone();
            new_rhs.pop();

            let new_equation = Self {
                lhs: self.lhs / last_element,
                rhs: new_rhs,
            };

            if new_equation.is_valid(extended_operator_set) {
                return true;
            }
        }

        // Check what happens if the last operation is addition
        let mut new_rhs = self.rhs.clone();
        new_rhs.pop();

        let new_equation = Self {
            lhs: self.lhs.saturating_sub(*last_element),
            rhs: new_rhs,
        };

        if new_equation.is_valid(extended_operator_set) {
            return true;
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    Some(
        equations
            .iter()
            .filter_map(|equation| match equation.is_valid(false) {
                true => Some(equation.lhs),
                false => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    Some(
        equations
            .iter()
            .filter_map(|equation| match equation.is_valid(true) {
                true => Some(equation.lhs),
                false => None,
            })
            .sum(),
    )
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(':');
            let lhs = parts.next().unwrap().parse::<u64>().unwrap();
            let rhs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            Equation { lhs, rhs }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
