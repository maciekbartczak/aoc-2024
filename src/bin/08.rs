use advent_of_code::{Grid, Vec2};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    get_antinodes_count(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_antinodes_count(input, true)
}

fn get_antinodes_count(input: &str, use_resonant_harmonics: bool) -> Option<u32> {
    let grid = Grid::from_input(input);
    let satellite_groups = parse_satellite_groups(&grid);

    let mut antinodes: HashSet<Vec2> = HashSet::new();
    for positions in satellite_groups.iter() {
        for position in positions {
            let rest = positions.iter().filter(|p| *p != position).collect();
            let satellite_antinodes: HashSet<Vec2> = if use_resonant_harmonics {
                antinodes.extend(positions);
                get_antinodes_with_resonant_harmonics(position, rest, &grid)
            } else {
                get_antinodes(position, rest, &grid)
            };

            antinodes.extend(&satellite_antinodes);
        }
    }

    Some(antinodes.len() as u32)
}

fn get_antinodes(satellite: &Vec2, other_satellites: Vec<&Vec2>, grid: &Grid) -> HashSet<Vec2> {
    other_satellites
        .iter()
        .filter_map(|other| {
            let distance = satellite.sub(&other);
            let antinode = satellite.add(&distance);

            if grid.is_point_in_gird(&antinode) {
                Some(antinode)
            } else {
                None
            }
        })
        .collect()
}

fn get_antinodes_with_resonant_harmonics(
    satellite: &Vec2,
    other_satellites: Vec<&Vec2>,
    grid: &Grid,
) -> HashSet<Vec2> {
    other_satellites
        .iter()
        .flat_map(|other| {
            let mut antinodes = HashSet::new();
            let distance = satellite.sub(&other);

            let mut antinode = satellite.add(&distance);
            while grid.is_point_in_gird(&antinode) {
                antinodes.insert(antinode);
                antinode = antinode.add(&distance);
            }

            antinodes
        })
        .collect()
}

fn parse_satellite_groups(grid: &Grid) -> Vec<HashSet<Vec2>> {
    let mut map = HashMap::new();

    for (pos, value) in grid.iter() {
        if value != '.' {
            map.entry(value)
                .or_insert_with(|| HashSet::new())
                .insert(pos);
        }
    }

    Vec::from_iter(map.into_values())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
