use advent_of_code::Vec2;

advent_of_code::solution!(13);

const PRIZE_OFFSET: i64 = 10000000000000;

#[derive(Debug)]
struct Machine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl Machine {
    pub fn solve(&self, offset_prize: bool) -> Option<u64> {
        // a*ax + b*bx = px
        // a*ay + b*by = py
        let ax = self.button_a.x as i64;
        let bx = self.button_b.x as i64;
        let ay = self.button_a.y as i64;
        let by = self.button_b.y as i64;
        let mut px: i64 = self.prize.x as i64;
        let mut py: i64 = self.prize.y as i64;

        if offset_prize {
            px += PRIZE_OFFSET;
            py += PRIZE_OFFSET;
        }

        let b = (py * ax - px * ay) / (by * ax - bx * ay);
        let a = (px - b * bx) / ax;

        if a * ax + b * bx == px && a * ay + b * by == py {
            return Some(a as u64 * 3 + b as u64);
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(machines.iter().filter_map(|m| m.solve(false)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(machines.iter().filter_map(|m| m.solve(true)).sum())
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|l| {
            let mut machine_lines = l.lines();
            let button_a_parts: Vec<&str> = machine_lines.next().unwrap().split(", ").collect();
            let button_b_parts: Vec<&str> = machine_lines.next().unwrap().split(", ").collect();
            let prize_parts: Vec<&str> = machine_lines.next().unwrap().split(", ").collect();

            let a_x = button_a_parts[0]
                .strip_prefix("Button A: X+")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let a_y = button_a_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let b_x = button_b_parts[0]
                .strip_prefix("Button B: X+")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let b_y = button_b_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let prize_x = prize_parts[0]
                .strip_prefix("Prize: X=")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let prize_y = prize_parts[1]
                .strip_prefix("Y=")
                .unwrap()
                .parse::<i32>()
                .unwrap();

            Machine {
                button_a: Vec2::new(a_x, a_y),
                button_b: Vec2::new(b_x, b_y),
                prize: Vec2::new(prize_x, prize_y),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908))
    }
}
