advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (winning_numbers, my_numbers) = parse_line(line);
        let winning_numbers_count = get_matching_numbers_count(&winning_numbers, &my_numbers);

        if winning_numbers_count > 0 {
            sum  += 1 << (winning_numbers_count - 1);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut card_counts = vec![1; line_count];

    for (index, line) in input.lines().enumerate() {
        let (winning_numbers, my_numbers) = parse_line(line);
        let winning_numbers_count = get_matching_numbers_count(&winning_numbers, &my_numbers);

        let max_index = std::cmp::min(line_count - 1, index + winning_numbers_count as usize);
        let current_card_count = card_counts[index];
        for add_index in (index + 1)..=max_index {
            card_counts[add_index] += current_card_count;
        }
    }

    Some(card_counts.iter().sum())
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parts = line.split(':');
    parts.next();
    let mut numbers = parts.next().unwrap().split('|');
    let winning_numbers = parse_number_line(numbers.next().unwrap());
    let my_numbers = parse_number_line(numbers.next().unwrap());
    (winning_numbers, my_numbers)
}

fn parse_number_line(line: &str) -> Vec<u32> {
    line.trim().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

fn get_matching_numbers_count(winning_numbers: &Vec<u32>, my_numbers: &Vec<u32>) -> u32 {
    let mut matching_numbers_count = 0;
    for winning_number in winning_numbers {
        if my_numbers.contains(&winning_number) {
            matching_numbers_count += 1;
        }
    }
    matching_numbers_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
