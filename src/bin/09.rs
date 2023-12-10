advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut sum = 0;

    for line in input.lines() {
        let numbers = parse_line(line);
        sum += get_next_number(&numbers);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut sum = 0;

    for line in input.lines() {
        let mut numbers = parse_line(line);
        numbers.reverse();
        sum += get_next_number(&numbers);
    }

    Some(sum)
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_next_number(line: &Vec<i64>) -> i64 {
    return if line.iter().all(|x| *x == 0) {
        0
    } else {
        let mut differences = Vec::with_capacity(line.len() - 1);
        for index in 1..line.len() {
            differences.push(line[index] - line[index - 1]);
        }
        line.last().unwrap() + get_next_number(&differences)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
