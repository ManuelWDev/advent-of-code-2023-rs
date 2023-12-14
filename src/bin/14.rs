use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    move_rocks(&mut map, &get_north, &set_north);
    Some(count_north_load(&map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);

    let mut maps = HashMap::new();
    let iteration_count = 1000000000;

    for i in 0..iteration_count {
        spin_rocks(&mut map);

        // check if pattern repeats
        if let Some(previous) = maps.insert(hash(&map), i) {
            let cycle = i - previous;
            let remaining = (iteration_count - i) % cycle - 1;
            for _ in 0..remaining {
                spin_rocks(&mut map);
            }
            return Some(count_north_load(&map));
        }
    }
    Some(count_north_load(&map))
}

fn hash(map: &Vec<Vec<RockType>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    map.hash(&mut hasher);
    hasher.finish()
}

fn parse(input: &str) -> Vec<Vec<RockType>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    None,
    Moving,
    Stationary,
}

impl From<char> for RockType {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::None,
            '#' => Self::Stationary,
            'O' => Self::Moving,
            _ => panic!("Invalid rock type: {}", c),
        }
    }
}

fn spin_rocks(map: &mut Vec<Vec<RockType>>) {
    move_rocks(map, &get_north, &set_north);
    move_rocks(map, &get_west, &set_west);
    move_rocks(map, &get_south, &set_south);
    move_rocks(map, &get_east, &set_east);
}

fn move_rocks(
    map: &mut Vec<Vec<RockType>>,
    get_map: &dyn Fn(&mut Vec<Vec<RockType>>, usize, usize) -> &RockType,
    set_map: &dyn Fn(&mut Vec<Vec<RockType>>, usize, usize, RockType),
) {
    for j in 0..map.len() {
        let mut free_index = 0;
        let mut free_count = 0;
        for i in 0..map.len() {
            match get_map(map, i, j) {
                RockType::None => {
                    if free_count == 0 {
                        free_index = i;
                    }
                    free_count += 1;
                }
                RockType::Stationary => {
                    free_count = 0;
                }
                RockType::Moving => {
                    if free_count > 0 {
                        let used_rock_index = free_index;
                        free_index += 1;

                        set_map(map, i, j, RockType::None);
                        set_map(map, used_rock_index, j, RockType::Moving);
                    }
                }
            }
        }
    }
}

fn count_north_load(map: &Vec<Vec<RockType>>) -> u32 {
    let mut sum = 0;
    for j in 0..map.len() {
        for i in 0..map.len() {
            match map[i][j] {
                RockType::Moving => {
                    sum += map.len() - i;
                }
                _ => {}
            }
        }
    }
    sum as u32
}

fn get_north(map: &mut Vec<Vec<RockType>>, i: usize, j: usize) -> &RockType {
    return &map[i][j];
}

fn set_north(map: &mut Vec<Vec<RockType>>, i: usize, j: usize, value: RockType) {
    map[i][j] = value;
}

fn get_south(map: &mut Vec<Vec<RockType>>, i: usize, j: usize) -> &RockType {
    return &map[map.len() - 1 - i][j];
}

fn set_south(map: &mut Vec<Vec<RockType>>, i: usize, j: usize, value: RockType) {
    let len = map.len() - 1;
    map[len - i][j] = value;
}

fn set_west(map: &mut Vec<Vec<RockType>>, i: usize, j: usize, value: RockType) {
    map[j][i] = value;
}

fn get_west(map: &mut Vec<Vec<RockType>>, i: usize, j: usize) -> &RockType {
    return &map[j][i];
}

fn set_east(map: &mut Vec<Vec<RockType>>, i: usize, j: usize, value: RockType) {
    let len = map.len() - 1;
    map[j][len - i] = value;
}

fn get_east(map: &mut Vec<Vec<RockType>>, i: usize, j: usize) -> &RockType {
    let len = map.len() - 1;
    return &map[j][len - i];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
