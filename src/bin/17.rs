use std::collections::BinaryHeap;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    Some(map.shortest_path())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    map.set_distances(4, 11);
    Some(map.shortest_path())
}

struct Map {
    tiles: Vec<Vec<u8>>,
    min_roll_distance: u8,
    max_roll_distance: u8,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            tiles: value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
            min_roll_distance: 1,
            max_roll_distance: 4,
        }
    }
}

impl Map {
    fn set_distances(&mut self, min_roll_distance: u8, max_roll_distance: u8) {
        self.min_roll_distance = min_roll_distance;
        self.max_roll_distance = max_roll_distance;
    }

    fn shortest_path(&self) -> u32 {
        let mut visit_infos: Vec<Vec<Vec<State>>> =
            vec![vec![Vec::new(); self.tiles[0].len()]; self.tiles.len()];

        let mut heap = BinaryHeap::new();

        heap.push(SearchState {
            position: Position::new(0, 0),
            state: State::new(0, Movement::Down(0)),
        });

        heap.push(SearchState {
            position: Position::new(0, 0),
            state: State::new(0, Movement::Right(0)),
        });

        let goal = Position::new(self.tiles[0].len() - 1, self.tiles.len() - 1);

        'outer: while let Some(SearchState { position, state }) = heap.pop() {
            if position == goal {
                if state.movement.length() < self.min_roll_distance {
                    continue;
                }
                return state.cost;
            }

            let visit_info = &mut visit_infos[position.y][position.x];

            let mut changed_a_state = false;
            
            for visit_state in visit_info.iter_mut() {
                if !visit_state.movement.variant_equals(&state.movement) {
                    continue;
                }
                if visit_state.movement.length() != state.movement.length() {
                    if visit_state.movement.length() >= self.min_roll_distance
                        && visit_state.movement.length() <= state.movement.length()
                        && visit_state.cost <= state.cost
                    {
                        continue 'outer;
                    }
                    continue;
                }
                if visit_state.cost <= state.cost {
                    continue 'outer;
                } else {
                    visit_state.cost = state.cost;
                    changed_a_state = true;
                }
            }

            if !changed_a_state {
                visit_info.push(state);
            }

            if let Some(new_state) = self.get_moved_state(&state, &position, &Movement::Up(1)) {
                heap.push(new_state);
            }
            if let Some(new_state) = self.get_moved_state(&state, &position, &Movement::Down(1)) {
                heap.push(new_state);
            }
            if let Some(new_state) = self.get_moved_state(&state, &position, &Movement::Left(1)) {
                heap.push(new_state);
            }
            if let Some(new_state) = self.get_moved_state(&state, &position, &Movement::Right(1)) {
                heap.push(new_state);
            }
        }
        0
    }

    fn get_moved_state(
        &self,
        state: &State,
        position: &Position,
        movement: &Movement,
    ) -> Option<SearchState> {
        if let Some(new_movement) =
            state
                .movement
                .change_movement(movement, self.min_roll_distance, self.max_roll_distance)
        {
            if let Some(new_position) =
                position.move_in_dir(movement, self.tiles.len() - 1, self.tiles[0].len() - 1)
            {
                let cost = self.tiles[new_position.y][new_position.x];
                return Some(SearchState {
                    position: new_position,
                    state: State::new(state.cost + cost as u32, new_movement),
                });
            }
            return None;
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    movement: Movement,
}

impl State {
    pub fn new(cost: u32, movement: Movement) -> Self {
        State { cost, movement }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SearchState {
    position: Position,
    state: State,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .state
            .cost
            .cmp(&self.state.cost)
            .then_with(|| {
                other
                    .state
                    .movement
                    .length()
                    .cmp(&self.state.movement.length())
            })
            .then_with(|| other.position.x.cmp(&self.position.x))
            .then_with(|| other.position.y.cmp(&self.position.y))
            .then_with(|| other.state.movement.cmp(&self.state.movement))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Movement {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl Movement {
    fn length(&self) -> u8 {
        match self {
            Self::Up(length) => *length,
            Self::Down(length) => *length,
            Self::Left(length) => *length,
            Self::Right(length) => *length,
        }
    }

    fn change_movement(
        &self,
        movement: &Movement,
        min_roll_distance: u8,
        max_roll_distance: u8,
    ) -> Option<Movement> {
        if self.is_opposite(movement) {
            return None;
        }
        if self.variant_equals(movement) {
            let new_movement = match self {
                Self::Up(length) => Self::Up(*length + 1),
                Self::Down(length) => Self::Down(*length + 1),
                Self::Left(length) => Self::Left(*length + 1),
                Self::Right(length) => Self::Right(*length + 1),
            };
            if new_movement.length() < max_roll_distance {
                return Some(new_movement);
            }
            None
        } else if self.length() < min_roll_distance {
            None
        } else {
            Some(*movement)
        }
    }

    fn variant_equals(&self, other: &Movement) -> bool {
        matches!(
            (self, other),
            (Self::Up(_), Self::Up(_))
                | (Self::Down(_), Self::Down(_))
                | (Self::Left(_), Self::Left(_))
                | (Self::Right(_), Self::Right(_))
        )
    }

    fn is_opposite(&self, other: &Movement) -> bool {
        matches!(
            (self, other),
            (Self::Up(_), Self::Down(_))
                | (Self::Down(_), Self::Up(_))
                | (Self::Left(_), Self::Right(_))
                | (Self::Right(_), Self::Left(_))
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_dir(&self, movement: &Movement, max_y: usize, max_x: usize) -> Option<Position> {
        match movement {
            Movement::Down(_) => {
                if self.y < max_y {
                    Some(Position::new(self.x, self.y + 1))
                } else {
                    None
                }
            }
            Movement::Up(_) => {
                if self.y > 0 {
                    Some(Position::new(self.x, self.y - 1))
                } else {
                    None
                }
            }
            Movement::Left(_) => {
                if self.x > 0 {
                    Some(Position::new(self.x - 1, self.y))
                } else {
                    None
                }
            }
            Movement::Right(_) => {
                if self.x < max_x {
                    Some(Position::new(self.x + 1, self.y))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
