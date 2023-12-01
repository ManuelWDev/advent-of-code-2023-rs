advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let normal_digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let search_digits = [normal_digits].to_vec();

    Some(calculate_line_sum(input, &search_digits))
}

pub fn part_two(input: &str) -> Option<u32> {
    let named_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let normal_digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let search_digits = [normal_digits, named_digits].to_vec();

    Some(calculate_line_sum(input, &search_digits))
}

struct FoundDigit {
    digit: u32,
    index: usize,
}

fn calculate_line_sum(lines: &str, search_digits: &Vec<[&str; 9]>) -> u32 {
    let mut sum = 0;
    for line in lines.lines() {
        sum += line_to_number(line, &search_digits);
    }
    sum
}

fn line_to_number(line: &str, search_digits: &Vec<[&str; 9]>) -> u32 {
    let mut found_digits = Vec::new();
    for search_digit in search_digits {
        found_digits.append(&mut find_digits(line, search_digit));
    }

    calculate_value(&mut found_digits)
}

fn calculate_value(found_digits: &mut Vec<FoundDigit>) -> u32 {
    found_digits.sort_by(|a, b| a.index.cmp(&b.index));

    let mut first_digit = 0;
    let mut last_digit = 0;

    if found_digits.len() > 0 {
        first_digit = found_digits[0].digit;
        last_digit = first_digit;
    }
    if found_digits.len() > 1 {
        last_digit = found_digits.iter().last().unwrap().digit;
    }

    first_digit * 10 + last_digit
}

fn find_digits(line: &str, named_digits: &[&str; 9]) -> Vec<FoundDigit> {
    let mut found_digits = Vec::new();

    for (digit_index, name) in named_digits.iter().enumerate() {
        found_digits.append(&mut line.match_indices(name).map(|(index, _)| FoundDigit {
            digit: (digit_index + 1) as u32,
            index,
        }).collect::<Vec<FoundDigit>>());
    }

    found_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_part2", DAY));
        assert_eq!(result, Some(281));
    }
}
