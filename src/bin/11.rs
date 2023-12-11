advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input, 1);
    Some(pair_distance_sum(&points))
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input, 999999);
    Some(pair_distance_sum(&points))
}

fn pair_distance_sum(points: &Vec<Point>) -> u64 {
    let mut distance_sum = 0;
    for (index, point) in points.iter().enumerate() {
        for point2 in &points[index + 1..] {
            let distance =
                (point.x as i64 - point2.x as i64).abs() + (point.y as i64 - point2.y as i64).abs();
            distance_sum += distance as u64;
        }
    }
    distance_sum
}

fn parse_input(input: &str, empty_line_length: usize) -> Vec<Point> {
    let column_count = input.lines().next().unwrap().len();
    let mut columns: Vec<Vec<Point>> = Vec::with_capacity(column_count);
    for _ in 0..column_count {
        columns.push(Vec::new());
    }

    let mut empty_row_length = 0;
    for (row, line) in input.lines().enumerate() {
        let mut something_in_row = false;
        for (column, character) in line.chars().enumerate() {
            match character {
                '#' => {
                    something_in_row = true;
                    columns[column].push(Point::new(column, row + empty_row_length));
                }
                _ => {}
            }
        }

        if !something_in_row {
            empty_row_length += empty_line_length;
        }
    }

    let mut empty_column_length = 0;
    for column in columns.iter_mut() {
        if column.is_empty() {
            empty_column_length += empty_line_length;
        }
        for point in column.iter_mut() {
            point.x += empty_column_length;
        }
    }

    columns.iter().flatten().cloned().collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
