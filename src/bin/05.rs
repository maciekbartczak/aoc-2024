use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Debug)]
struct PageOrderingRules {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    lhs: u32,
    rhs: u32,
}

type Update = Vec<u32>;

#[derive(Debug)]
struct Data {
    rules: PageOrderingRules,
    updates: Vec<Update>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(&input);

    Some(
        data.updates
            .iter()
            .filter(|update| is_valid(&data.rules, &create_pages_map(update)))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(&input);

    Some(
        data.updates
            .iter()
            .filter(|update| !is_valid(&data.rules, &create_pages_map(update)))
            .map(|update| make_valid(update, &data.rules, &create_pages_map(update)))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

fn parse_input(input: &str) -> Data {
    let mut input_parts = input.split("\n\n");
    let rules_part = input_parts.next().unwrap();
    let updates_part = input_parts.next().unwrap();

    Data {
        rules: parse_rules(rules_part),
        updates: parse_updates(updates_part),
    }
}

fn parse_updates(updates: &str) -> Vec<Update> {
    updates
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn parse_rules(rules: &str) -> PageOrderingRules {
    let rules = rules
        .lines()
        .map(|rule| {
            let mut rule_parts = rule.split('|');
            let lhs = rule_parts.next().unwrap().parse::<u32>().unwrap();
            let rhs = rule_parts.next().unwrap().parse::<u32>().unwrap();

            Rule { lhs, rhs }
        })
        .collect();

    PageOrderingRules { rules }
}

fn is_valid(rules: &PageOrderingRules, page_number_to_index_map: &HashMap<&u32, usize>) -> bool {
    for rule in &rules.rules {
        if !page_number_to_index_map.contains_key(&rule.lhs) {
            continue;
        }
        if !page_number_to_index_map.contains_key(&rule.rhs) {
            continue;
        }
        let lhs_idx = page_number_to_index_map.get(&rule.lhs).unwrap();
        let rhs_idx = page_number_to_index_map.get(&rule.rhs).unwrap();

        if lhs_idx > rhs_idx {
            return false;
        }
    }

    true
}

fn create_pages_map(update: &Update) -> HashMap<&u32, usize> {
    let mut page_number_to_index_map = HashMap::new();

    for (idx, page) in update.iter().enumerate() {
        page_number_to_index_map.insert(page, idx);
    }

    page_number_to_index_map
}

fn make_valid(
    update: &Update,
    rules: &PageOrderingRules,
    pages_map: &HashMap<&u32, usize>,
) -> Update {
    let mut page_number_to_index_map = pages_map.clone();
    let mut new_update = update.clone();

    for rule in &rules.rules {
        if !page_number_to_index_map.contains_key(&rule.lhs) {
            continue;
        }
        if !page_number_to_index_map.contains_key(&rule.rhs) {
            continue;
        }
        let lhs_idx = *page_number_to_index_map.get(&rule.lhs).unwrap();
        let rhs_idx = *page_number_to_index_map.get(&rule.rhs).unwrap();

        if lhs_idx > rhs_idx {
            new_update.swap(lhs_idx, rhs_idx);
            page_number_to_index_map.insert(&rule.lhs, rhs_idx);
            page_number_to_index_map.insert(&rule.rhs, lhs_idx);
        }
    }

    if !is_valid(rules, &page_number_to_index_map) {
        return make_valid(&new_update, rules, &page_number_to_index_map);
    }

    new_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
