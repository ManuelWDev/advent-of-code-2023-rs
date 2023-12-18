advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let movements = parse_instructions_part1(input);
    let map = Map::from(&movements);
    Some(map.get_area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let movements = parse_instructions_part2(input);
    let map = Map::from(&movements);
    Some(map.get_area())
}

fn parse_instructions_part1(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap().chars().next().unwrap();
            let length = parts.next().unwrap();
            (direction, length).into()
        })
        .collect()
}

fn parse_instructions_part2(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace();
            let total_color_part = parts.last().unwrap();
            let used_color_part = &total_color_part[1..(total_color_part.len() - 1)];
            let length = &used_color_part[1..=5];
            let direction_index = used_color_part
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();

            (direction_index, length).into()
        })
        .collect()
}

struct Map {
    points: Vec<Point>,
    total_length: u64,
}

impl Map {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    fn get_area(&self) -> u64 {
        let mut sum: i64 = 0;
        let points = &self.points;
        for i in 1..points.len() {
            let point = points[i];
            let last_point = points[i - 1];
            let next_point = points[(i + 1) % points.len()];

            sum += point.x * (next_point.y - last_point.y);
        }

        let size_from_points_themselves = self.total_length / 2 + 1;
        (sum / 2) as u64 + size_from_points_themselves
    }
}

impl From<&Vec<Movement>> for Map {
    fn from(instructions: &Vec<Movement>) -> Self {
        let mut location = Point::new();
        let mut points = Vec::new();
        points.push(location);

        let mut total_length = 0;

        for movement in instructions {
            match movement {
                Movement::Up(steps) => {
                    location.y -= *steps as i64;
                    total_length += *steps as u64;
                    points.push(location)
                }
                Movement::Down(steps) => {
                    location.y += *steps as i64;
                    total_length += *steps as u64;
                    points.push(location)
                }
                Movement::Left(steps) => {
                    location.x -= *steps as i64;
                    total_length += *steps as u64;
                    points.push(location)
                }
                Movement::Right(steps) => {
                    location.x += *steps as i64;
                    total_length += *steps as u64;
                    points.push(location)
                }
            }
        }
        Map {
            points,
            total_length,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl From<(char, &str)> for Movement {
    fn from(move_infos: (char, &str)) -> Self {
        let (direction, steps) = move_infos;
        let steps = steps.parse::<u32>().unwrap();
        match direction {
            'U' => Movement::Up(steps),
            'D' => Movement::Down(steps),
            'L' => Movement::Left(steps),
            'R' => Movement::Right(steps),
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<(u32, &str)> for Movement {
    fn from(move_infos: (u32, &str)) -> Self {
        let (direction, steps) = move_infos;
        let steps = i64::from_str_radix(steps, 16).unwrap() as u32;
        match direction {
            0 => Movement::Right(steps),
            1 => Movement::Down(steps),
            2 => Movement::Left(steps),
            3 => Movement::Up(steps),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
