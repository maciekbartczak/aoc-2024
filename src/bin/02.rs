advent_of_code::solution!(2);

type Report = Vec<u32>;

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .iter()
            .filter(|report| is_safe(report, false))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .iter()
            .filter(|report| is_safe(report, true))
            .count() as u32,
    )
}

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &Report, try_remove_bad_level: bool) -> bool {
    assert!(report.len() >= 2);

    #[derive(PartialEq)]
    enum ReportOrder {
        Ascending,
        Descending,
    }
    let expected_order = match report[0].cmp(&report[1]) {
        std::cmp::Ordering::Less => ReportOrder::Ascending,
        std::cmp::Ordering::Greater => ReportOrder::Descending,
        std::cmp::Ordering::Equal => return can_report_be_made_safe(report, try_remove_bad_level),
    };
    let expected_range = 1..=3;

    for i in 0..report.len() - 1 {
        let left_level = report[i];
        let right_level = report[i + 1];

        let order = match left_level.cmp(&right_level) {
            std::cmp::Ordering::Less => ReportOrder::Ascending,
            std::cmp::Ordering::Greater => ReportOrder::Descending,
            std::cmp::Ordering::Equal => {
                return can_report_be_made_safe(report, try_remove_bad_level)
            }
        };
        if order != expected_order {
            return can_report_be_made_safe(report, try_remove_bad_level);
        }

        let diff = left_level.abs_diff(right_level);
        if !expected_range.contains(&diff) {
            return can_report_be_made_safe(report, try_remove_bad_level);
        }
    }

    true
}

fn can_report_be_made_safe(report: &Report, try_remove_bad_level: bool) -> bool {
    if try_remove_bad_level {
        for i in 0..report.len() {
            let mut left_level_removed = report.clone();
            left_level_removed.remove(i);
            if is_safe(&left_level_removed, false) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
