advent_of_code::solution!(3);

struct MarkChar {
    char: char,
    marked: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = parse_input(input);
    Some(marked_number_sum(&rows))
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = parse_input(input);
    Some(calc_gear_ratio_sum(&rows))
}

fn parse_input(input: &str) -> Vec<Vec<MarkChar>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        rows.push(line.chars().map(|c| MarkChar { char: c, marked: false }).collect::<Vec<MarkChar>>());
    }

    for row_index in 0..rows.len() {
        for index in 0..rows[row_index].len() {
            let mark_char = &mut rows[row_index][index];

            if mark_char.char.is_digit(10) || mark_char.char == '.' {
                continue;
            }

            for i in (row_index - 1)..=(row_index + 1) {
                for j in (index - 1)..=(index + 1) {
                    mark_if_exists(&mut rows, i, j);
                }
            }
        }
    }
    rows
}

fn marked_number_sum(rows: &Vec<Vec<MarkChar>>) -> u32 {
    let mut sum = 0;
    for row in rows {
        row.split(|mc| !mc.char.is_ascii_digit())
            .for_each(|number_row| {
                let mut any_marked = false;
                let mut number = 0;
                for mc in number_row {
                    number *= 10;
                    number += mc.char.to_digit(10).unwrap();
                    if mc.marked {
                        any_marked = true;
                    }
                }
                if any_marked {
                    sum += number;
                }
            });
    }
    sum
}

fn mark_if_exists(rows: &mut Vec<Vec<MarkChar>>, row_index: usize, index: usize) {
    if let Some(mc) = rows.get_mut(row_index).and_then(|row| row.get_mut(index)) {
        mc.marked = true;
    }
}

fn is_number_part(rows: &Vec<Vec<MarkChar>>, row_index: usize, index: usize) -> bool {
    match rows.get(row_index).and_then(|row| row.get(index)) {
        Some(mc) => mc.char.is_ascii_digit(),
        None => false,
    }
}

fn calc_gear_ratio_sum(rows: &Vec<Vec<MarkChar>>) -> u32 {
    let mut sum = 0;
    for (row_index, row) in rows.iter().enumerate() {
        for (index, mc) in row.iter().enumerate() {
            if !(mc.char == '*') {
                continue;
            }
            let mut already_has_one_number = false;
            let mut current_number = 0;

            'outer: for i in (row_index - 1)..=(row_index + 1) {
                for j in (index - 1)..=(index + 1) {
                    if is_number_part(rows, i, j) {
                        let number = get_full_number(rows, i, j);
                        if already_has_one_number {
                            sum += current_number * number;
                            continue 'outer;
                        }
                        else {
                            current_number = number;
                            already_has_one_number = true;
                        }

                        if j == index - 1 && is_number_part(rows, i, index) {
                            continue 'outer;
                        }

                        if j == index {
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    sum
}

fn get_full_number(rows: &Vec<Vec<MarkChar>>, row_index: usize, index: usize) -> u32 {
    let mut index = index;
    while index > 0 && is_number_part(rows, row_index, index - 1) {
        index -= 1;
    }

    let mut number = 0;
    for i in index..rows[row_index].len() {
        if !is_number_part(rows, row_index, i) {
            break;
        }
        number *= 10;
        number += rows[row_index][i].char.to_digit(10).unwrap();
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
