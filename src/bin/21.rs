use std::collections::HashSet;
advent_of_code::solution!(21);

#[cfg(test)]
const REQUIRED_STEPS: usize = 6;

#[cfg(not(test))]
const REQUIRED_STEPS: usize = 64;

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = parse(input);
    let mut indices = HashSet::new();
    indices.insert(start);

    for _ in 0..REQUIRED_STEPS {
        indices = execute_step(&map, indices);
    }

    Some(indices.len() as u32)
}

pub fn part_two(_: &str) -> Option<u32> {
    // start seems to be in the center of map
    // there seem to be "empty" horizontal and vertical "roads" in the input
    // so we somehow have to sum all the repeating tiles and take care of the edge cases of the diamond shape
    // maybe I will solve this later
    None
}

fn execute_step(map: &Map, indices: HashSet<Index>) -> HashSet<Index> {
    let mut new_indices = HashSet::new();
    for index in indices {
        for legal_indices in map.neighbours(&index) {
            new_indices.insert(legal_indices);
        }
    }
    new_indices
}

fn parse(input: &str) -> (Map, Index) {
    let mut index = Index::new(0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        index = Index::new(x, y);
                        true
                    }
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect();
    (Map::new(map), index)
}

struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn new(map: Vec<Vec<bool>>) -> Self {
        Map { map }
    }

    fn neighbours<'a>(&'a self, index: &'a Index) -> impl Iterator<Item = Index> + 'a {
        Direction::DIRECTIONS
            .iter()
            .filter_map(move |direction| self.remove_illegal(index.move_in_direction(direction)))
    }

    fn remove_illegal(&self, index: Index) -> Option<Index> {
        if index.x >= self.map.len() {
            return None;
        }
        if index.y >= self.map.len() {
            return None;
        }

        if self.map[index.y][index.x] {
            Some(index)
        } else {
            None
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Index {
    x: usize,
    y: usize,
}

impl Index {
    fn new(x: usize, y: usize) -> Self {
        Index { x, y }
    }

    fn move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Index::new(self.x, self.y - 1),
            Direction::Down => Index::new(self.x, self.y + 1),
            Direction::Left => Index::new(self.x - 1, self.y),
            Direction::Right => Index::new(self.x + 1, self.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
