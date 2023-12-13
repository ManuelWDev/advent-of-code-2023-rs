use std::cmp::min;
advent_of_code::solution!(13);

#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &'static str = "\n\n";

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse(input);
    let mut result = 0;

    for pattern in patterns.iter() {
        let horizontal_indices = get_horizontal_split_indices(pattern);
        if horizontal_indices.len() > 0 {
            result += 100 * horizontal_indices[0] as u32;
            continue;
        }

        let vertical_indices = get_vertical_split_indices(pattern);
        if vertical_indices.len() > 0 {
            result += vertical_indices[0] as u32;
            continue;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut patterns = parse(input);
    let mut result = 0;

    'outer: for pattern in patterns.iter_mut() {
        let old_horizontal = get_horizontal_split_indices(pattern);
        let old_vertical = get_vertical_split_indices(pattern);

        // this is a brute force approach, but runs under 1 millisecond
        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                pattern[i][j] = !pattern[i][j];

                let new_horizontal = get_horizontal_split_indices(pattern);
                if let Some(horizontal) = get_new_value(&new_horizontal, &old_horizontal) {
                    result += 100 * horizontal as u32;
                    continue 'outer;
                }

                let new_vertical = get_vertical_split_indices(pattern);
                if let Some(vertical) = get_new_value(&new_vertical, &old_vertical) {
                    result += vertical as u32;
                    continue 'outer;
                }

                pattern[i][j] = !pattern[i][j];
            }
        }
    }

    Some(result)
}

fn parse(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .split(EMPTY_LINE)
        .map(|chunk| parse_chunk(chunk))
        .collect::<Vec<_>>()
}

fn parse_chunk(chunk: &str) -> Vec<Vec<bool>> {
    chunk
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_new_value(new: &Vec<usize>, old: &Vec<usize>) -> Option<usize> {
    for new_value in new {
        if !old.contains(new_value) {
            return Some(*new_value);
        }
    }
    None
}

fn get_horizontal_split_indices(pattern: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut result = Vec::new();
    for horizontal_index in 0..pattern.len() {
        if is_mirror_horizontal(pattern, horizontal_index) {
            result.push(horizontal_index + 1);
        }
    }
    result
}

fn get_vertical_split_indices(pattern: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut result = Vec::new();
    for vertical_index in 0..pattern[0].len() {
        if is_mirror_vertical(pattern, vertical_index) {
            result.push(vertical_index + 1)
        }
    }
    result
}

fn is_mirror_horizontal(pattern: &Vec<Vec<bool>>, index: usize) -> bool {
    let last_index = pattern.len() - 2;
    if index > last_index {
        return false;
    }

    let max_distance = min(last_index - index, index);
    for index_change in 0..=max_distance {
        if &pattern[index - index_change] != &pattern[index + index_change + 1] {
            return false;
        }
    }
    return true;
}

fn is_mirror_vertical(pattern: &Vec<Vec<bool>>, index: usize) -> bool {
    let horizontal_len = pattern[0].len();
    let last_index = horizontal_len - 2;
    if index > last_index {
        return false;
    }

    let max_distance = min(last_index - index, index);
    for index_change in 0..=max_distance {
        for row in pattern.iter() {
            if &row[index - index_change] != &row[index + index_change + 1] {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
