use std::collections::{HashSet};
advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    Some(map.get_longest_path())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    map.set_steep_relevant(false);
    // this is a really bad bruteforce approach for the second part
    // we could create a graph in which consecutive nodes are connected to edges
    Some(map.get_longest_path())
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Index,
    end: Index,
    steep_relevant: bool,
}

impl Map {
    fn set_steep_relevant(&mut self, steep_relevant: bool) {
        self.steep_relevant = steep_relevant;
    }

    fn get_longest_path(&self) -> u32 {
        self.get_longest_path_rec(self.start, &mut HashSet::new()) as u32 + 1
    }

    fn get_longest_path_rec(&self, current: Index, visited: &mut HashSet<Index>) -> i64 {
        if current == self.end {
            return 1;
        }

        if visited.contains(&current) {
            return i64::MIN;
        }

        visited.insert(current);

        let tile = &self.tiles[current.y][current.x];
        if tile == &Tile::Wall {
            return i64::MIN;
        }

        let mut max = i64::MIN;
        if self.steep_relevant {
            if tile == &Tile::Empty {
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x - 1, current.y), visited),
                    max,
                );
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x + 1, current.y), visited),
                    max,
                );
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x, current.y - 1), visited),
                    max,
                );
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x, current.y + 1), visited),
                    max,
                );
            }
            if self.tiles[current.y][current.x] == Tile::Left {
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x - 1, current.y), visited),
                    max,
                );
            }
            if self.tiles[current.y][current.x] == Tile::Right {
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x + 1, current.y), visited),
                    max,
                );
            }
            if self.tiles[current.y][current.x] == Tile::Up {
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x, current.y - 1), visited),
                    max,
                );
            }
            if self.tiles[current.y][current.x] == Tile::Down {
                max = i64::max(
                    self.get_longest_path_rec(Index::new(current.x, current.y + 1), visited),
                    max,
                );
            }
        } else {
            max = i64::max(
                self.get_longest_path_rec(Index::new(current.x - 1, current.y), visited),
                max,
            );
            max = i64::max(
                self.get_longest_path_rec(Index::new(current.x + 1, current.y), visited),
                max,
            );
            max = i64::max(
                self.get_longest_path_rec(Index::new(current.x, current.y - 1), visited),
                max,
            );
            max = i64::max(
                self.get_longest_path_rec(Index::new(current.x, current.y + 1), visited),
                max,
            );
        }

        visited.remove(&current);

        max + 1
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut all_inputs: Vec<Vec<Tile>> = value
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        let start_x = all_inputs[0]
            .iter()
            .position(|c| c == &Tile::Empty)
            .unwrap();
        let end_x = all_inputs[all_inputs.len() - 1]
            .iter()
            .position(|c| c == &Tile::Empty)
            .unwrap();

        let rows = all_inputs.len() - 1;
        all_inputs[0][start_x] = Tile::Wall;
        all_inputs[rows][end_x] = Tile::Wall;

        Self {
            tiles: all_inputs,
            start: Index::new(start_x, 1),
            end: Index::new(end_x, rows - 1),
            steep_relevant: true,
        }
    }
}

impl Map {}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Index {
    x: usize,
    y: usize,
}

impl Index {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Tile {
    fn from(s: char) -> Self {
        match s {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '<' => Tile::Left,
            '>' => Tile::Right,
            '^' => Tile::Up,
            'v' => Tile::Down,
            _ => panic!("Invalid tile: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
