use std::cmp::max;
advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    move_light(&mut map, &Index::new(-1, 0), &Direction::Right);
    Some(count_energized_tiles(&map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut max_energy = 0;

    let len = map.len() as i32;
    for x in 0..map.len() {
        max_energy = max(
            max_energy,
            get_energy(&map, Index::new(x as i32, -1), Direction::Down),
        );
        max_energy = max(
            max_energy,
            get_energy(&map, Index::new(x as i32, len), Direction::Up),
        );
        max_energy = max(
            max_energy,
            get_energy(&map, Index::new(-1, x as i32), Direction::Right),
        );
        max_energy = max(
            max_energy,
            get_energy(&map, Index::new(len, x as i32), Direction::Left),
        );
    }

    Some(max_energy)
}

fn get_energy(map: &Vec<Vec<Tile>>, start_index: Index, start_direction: Direction) -> u32 {
    let mut map = map.clone();
    move_light(&mut map, &start_index, &start_direction);
    count_energized_tiles(&map)
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|c| Tile::new(c)).collect()
}

fn move_light(map: &mut Vec<Vec<Tile>>, index: &Index, direction: &Direction) {
    let new_index = direction.next_index(index);

    if !new_index.is_valid(map.len() as i32) {
        return;
    }

    let tile = &mut map[new_index.y as usize][new_index.x as usize];

    if tile.is_energized(direction) {
        return;
    }

    tile.set_energized(direction);

    match (tile.kind, direction) {
        ('.', _) => move_light(map, &new_index, direction),
        ('\\', Direction::Left) => move_light(map, &new_index, &Direction::Up),
        ('\\', Direction::Right) => move_light(map, &new_index, &Direction::Down),
        ('\\', Direction::Up) => move_light(map, &new_index, &Direction::Left),
        ('\\', Direction::Down) => move_light(map, &new_index, &Direction::Right),
        ('/', Direction::Left) => move_light(map, &new_index, &Direction::Down),
        ('/', Direction::Right) => move_light(map, &new_index, &Direction::Up),
        ('/', Direction::Up) => move_light(map, &new_index, &Direction::Right),
        ('/', Direction::Down) => move_light(map, &new_index, &Direction::Left),
        ('-', Direction::Left | Direction::Right) => move_light(map, &new_index, direction),
        ('-', Direction::Up | Direction::Down) => {
            move_light(map, &new_index, &Direction::Left);
            move_light(map, &new_index, &Direction::Right);
        }
        ('|', Direction::Up | Direction::Down) => move_light(map, &new_index, direction),
        ('|', Direction::Left | Direction::Right) => {
            move_light(map, &new_index, &Direction::Up);
            move_light(map, &new_index, &Direction::Down);
        }
        _ => panic!("Invalid tile: {:?}", tile.kind),
    }
}

fn count_energized_tiles(map: &Vec<Vec<Tile>>) -> u32 {
    map.iter()
        .map(|row| row.iter().filter(|tile| tile.is_any_energized()).count() as u32)
        .sum()
}

struct Index {
    x: i32,
    y: i32,
}

impl Index {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_valid(&self, max_exclusive: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_exclusive && self.y < max_exclusive
    }
}

#[derive(Debug, Clone)]
struct Tile {
    kind: char,
    energized_left: bool,
    energized_right: bool,
    energized_up: bool,
    energized_down: bool,
}

impl Tile {
    fn new(kind: char) -> Self {
        Self {
            kind,
            energized_left: false,
            energized_right: false,
            energized_up: false,
            energized_down: false,
        }
    }

    fn is_any_energized(&self) -> bool {
        self.energized_left || self.energized_right || self.energized_up || self.energized_down
    }

    fn is_energized(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => self.energized_up,
            Direction::Down => self.energized_down,
            Direction::Left => self.energized_left,
            Direction::Right => self.energized_right,
        }
    }

    fn set_energized(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.energized_up = true,
            Direction::Down => self.energized_down = true,
            Direction::Left => self.energized_left = true,
            Direction::Right => self.energized_right = true,
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
    fn next_index(&self, index: &Index) -> Index {
        match self {
            Self::Up => Index::new(index.x, index.y - 1),
            Self::Down => Index::new(index.x, index.y + 1),
            Self::Left => Index::new(index.x - 1, index.y),
            Self::Right => Index::new(index.x + 1, index.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
