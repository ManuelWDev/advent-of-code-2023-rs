use std::collections::HashMap;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (remaining, required) = parse_line(line);
        sum += count_legal_permutations(remaining, &required, 0, &mut HashMap::new());
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let (remaining, required) = parse_line(line);

        let mut remain_str = remaining.to_string();
        remain_str.push('?');
        let mut multiplied_remaining = remain_str.repeat(5);
        multiplied_remaining.pop();

        let required_multiplied = required.repeat(5);

        sum += count_legal_permutations(
            &multiplied_remaining,
            &required_multiplied,
            0,
            &mut HashMap::new(),
        );
    }
    Some(sum)
}

fn parse_line(input: &str) -> (&str, Vec<u32>) {
    let mut parts = input.split_whitespace();
    let remaining = parts.next().unwrap();

    let required = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    (remaining, required)
}

fn count_legal_permutations<'a>(
    remaining: &'a str,
    required: &'a [u32],
    current_active: u32,
    cache: &mut HashMap<(&'a str, &'a [u32], u32), u64>,
) -> u64 {
    if remaining.is_empty() {
        return if required.is_empty() {
            if current_active == 0 {
                1
            } else {
                0
            }
        } else {
            if required.len() == 1 && required[0] == current_active {
                1
            } else {
                0
            }
        };
    }
    if required.is_empty() {
        if current_active != 0 {
            return 0;
        }
    } else {
        if required[0] < current_active {
            return 0;
        }
    }

    let key = (remaining, required, current_active);
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    return match remaining.chars().next().unwrap() {
        '.' => count_with_empty(remaining, required, current_active, cache),
        '#' => count_with_full(remaining, required, current_active, cache),
        '?' => {
            let count = count_with_full(remaining, required, current_active, cache)
                + count_with_empty(remaining, required, current_active, cache);
            cache.insert(key, count);
            count
        }
        _ => panic!("unknown char"),
    };
}

fn count_with_empty<'a>(
    remaining: &'a str,
    required: &'a [u32],
    current_active: u32,
    cache: &mut HashMap<(&'a str, &'a [u32], u32), u64>,
) -> u64 {
    let (used_required, used_current_count) =
        if !required.is_empty() && required[0] == current_active {
            (&required[1..], 0)
        } else {
            (required, current_active)
        };

    let count = if used_current_count != 0 {
        0
    } else {
        count_legal_permutations(&remaining[1..], used_required, 0, cache)
    };

    cache.insert((remaining, required, current_active), count);
    count
}

fn count_with_full<'a>(
    remaining: &'a str,
    required: &'a [u32],
    current_active: u32,
    cache: &mut HashMap<(&'a str, &'a [u32], u32), u64>,
) -> u64 {
    let count = count_legal_permutations(&remaining[1..], required, current_active + 1, cache);
    cache.insert((remaining, required, current_active), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_hard() {
        let result = part_one("?###???????? 3,2,1");
        assert_eq!(result, Some(10))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
