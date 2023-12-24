use std::ops::RangeInclusive;
advent_of_code::solution!(24);

#[cfg(test)]
const TARGET_RANGE: RangeInclusive<f64> = 7.0..=27.0;

#[cfg(not(test))]
const TARGET_RANGE: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse(input);
    Some(count_xy_collisions_in_range(&lines))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    for line in input.lines() {
        let mut parts = line.split(" @ ");
        let start = parts.next().unwrap().into();
        let direction = parts.next().unwrap().into();

        lines.push(Line::new(start, direction));
    }

    lines
}

fn count_xy_collisions_in_range(lines: &Vec<Line>) -> u32 {
    let mut count = 0;
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if let Some((x, y)) = lines[i].get_xy_collision(&lines[j]) {
                if TARGET_RANGE.contains(&x) && TARGET_RANGE.contains(&y) {
                    count += 1;
                }
            }
        }
    }

    count
}

struct Line {
    start: Vector,
    direction: Vector,
}

impl Line {
    fn new(start: Vector, direction: Vector) -> Self {
        Self { start, direction }
    }

    fn get_xy_collision(&self, other: &Line) -> Option<(f64, f64)> {
        let dx = other.start.x - self.start.x;
        let dy = other.start.y - self.start.y;
        let det = self.direction.y * other.direction.x - self.direction.x * other.direction.y;
        if det == 0 {
            return None;
        }
        let tx = (dy * other.direction.x - dx * other.direction.y) as f64 / (det as f64);
        let ty = (dy * self.direction.x - dx * self.direction.y) as f64 / (det as f64);

        if tx < 0.0 || ty < 0.0 {
            return None;
        }

        let intersect_x = self.start.x as f64 + tx * self.direction.x as f64;
        let intersect_y = self.start.y as f64 + tx * self.direction.y as f64;
        Some((intersect_x, intersect_y))
    }
}

struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();
        let z = parts.next().unwrap().trim().parse().unwrap();
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
