use std::cell::RefCell;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    Some(map.find_furthest_distance_in_loop(false))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from(input);
    map.find_furthest_distance_in_loop(true);
    Some(map.get_encircled_count())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const START_DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn to_opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn from_char(c: char) -> Option<[Direction; 2]> {
        match c {
            '|' => Some([Direction::Up, Direction::Down]),
            '-' => Some([Direction::Left, Direction::Right]),
            'L' => Some([Direction::Up, Direction::Right]),
            'J' => Some([Direction::Up, Direction::Left]),
            '7' => Some([Direction::Down, Direction::Left]),
            'F' => Some([Direction::Down, Direction::Right]),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

impl std::ops::Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = rhs.to_offset();
        Location {
            x: (self.x as i32 + x) as usize,
            y: (self.y as i32 + y) as usize,
        }
    }
}

struct Tile {
    directions: Option<[Direction; 2]>,
    is_solution_path: RefCell<bool>,
}

impl Tile {
    fn is_top_vertical(&self) -> bool {
        match &self.directions {
            Some(directions) => directions[0] == Direction::Up || directions[1] == Direction::Up,
            None => false,
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Location,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut tiles = Vec::new();
        let mut start = None;
        for (y, line) in value.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let directions = Direction::from_char(c);
                if c == 'S' {
                    start = Some(Location { x, y });
                }
                row.push(Tile {
                    directions,
                    is_solution_path: RefCell::new(false),
                });
            }
            tiles.push(row);
        }

        Self {
            tiles,
            start: start.unwrap(),
        }
    }
}

impl Map {
    fn is_connected(&self, location: Location, direction: Direction) -> bool {
        let (x, y) = direction.to_offset();
        let new_x = location.x as i32 + x;
        let new_y = location.y as i32 + y;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let x = new_x as usize;
        let y = new_y as usize;

        if y >= self.tiles.len() || x >= self.tiles[y].len() {
            return false;
        }

        let tile = &self.tiles[y][x];
        let opposite_direction = direction.to_opposite();
        match &tile.directions {
            Some(directions) => {
                if directions[0] == opposite_direction {
                    return true;
                }
                if directions[1] == opposite_direction {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    fn find_furthest_distance_in_loop(&self, paint_solution_path: bool) -> u32 {
        for direction in Direction::START_DIRECTIONS.iter() {
            let mut distance = 1;
            if self.try_apply_on_loop(self.start, *direction, &mut |_: &Tile| {
                distance += 1;
            }) {
                if paint_solution_path {
                    self.try_apply_on_loop(self.start, *direction, &mut |tile| {
                        tile.is_solution_path.replace(true);
                    });
                }
                return distance / 2;
            }
        }
        return 0;
    }

    fn try_apply_on_loop(
        &self,
        from: Location,
        dir: Direction,
        apply_to_tile: &mut dyn FnMut(&Tile) -> (),
    ) -> bool {

        let mut from = from;
        let mut dir = dir;

        loop {
            apply_to_tile(&self.tiles[from.y][from.x]);
            if from + dir == self.start {
                return true;
            }
            if let Some((new_location, new_direction)) = self.get_next(from, dir) {
                from = new_location;
                dir = new_direction;
            } else {
                return false;
            }
        }
    }

    fn get_next(&self, location: Location, direction: Direction) -> Option<(Location, Direction)> {
        if !self.is_connected(location, direction) {
            return None;
        }

        let new_location = location + direction;
        let new_tile = &self.tiles[new_location.y][new_location.x];

        let opposite_direction = direction.to_opposite();
        let new_directions = new_tile.directions.unwrap();
        let used_direction = if new_directions[0] == opposite_direction {
            new_directions[1]
        } else {
            new_directions[0]
        };

        Some((new_location, used_direction))
    }

    fn get_encircled_count(&self) -> u32 {
        let mut count = 0;
        for row in self.tiles.iter() {
            let mut correct_path_count = 0;
            for tile in row.iter() {
                if tile.is_solution_path.borrow().clone() {
                    // we have to check this for the edge-case that the path is on the row
                    if tile.is_top_vertical() {
                        correct_path_count += 1;
                    }
                    continue;
                }
                if correct_path_count % 2 != 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_part2", DAY));
        assert_eq!(result, Some(10));
    }
}
