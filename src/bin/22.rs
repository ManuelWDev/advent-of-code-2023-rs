use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicU32, Ordering};
advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    Some(map.fall_and_get_deletable_counts())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::from(input);
    map.fall_and_get_deletable_counts();
    Some(map.get_total_destruction_fall_count())
}

struct Map {
    cubes: Vec<Cube>,
    dependencies: HashMap<u32, Vec<u32>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut cubes: Vec<Cube> = value.lines().map(|line| line.into()).collect::<Vec<_>>();
        cubes.sort_by(|a, b| a.z.start().cmp(b.z.start()));
        Self {
            cubes,
            dependencies: HashMap::new(),
        }
    }
}

impl Map {
    fn fall_and_get_deletable_counts(&mut self) -> u32 {
        let mut deletable_cubes: HashSet<u32> =
            HashSet::from_iter(self.cubes.iter().map(|cube| cube.id));
        let mut filled_heights = vec![vec![(0, None); 10]; 10];
        let mut dependencies: HashMap<u32, Vec<u32>> = HashMap::new();

        for cube in &self.cubes {
            let mut highest_z = 0;
            let mut holding_cubes = HashSet::new();
            for x in cube.x.clone() {
                for y in cube.y.clone() {
                    let filled_height = &filled_heights[x as usize][y as usize];
                    if filled_height.0 > highest_z {
                        highest_z = filled_height.0;
                        holding_cubes.clear();
                    }
                    if filled_height.0 == highest_z {
                        if let Some(id) = filled_height.1 {
                            holding_cubes.insert(id);
                        }
                    }
                }
            }
            dependencies.insert(cube.id, holding_cubes.iter().cloned().collect());
            if holding_cubes.len() == 1 {
                deletable_cubes.remove(&holding_cubes.iter().next().unwrap());
            }
            for x in cube.x.clone() {
                for y in cube.y.clone() {
                    filled_heights[x as usize][y as usize] =
                        (highest_z + cube.height(), Some(cube.id));
                }
            }
        }

        self.dependencies = dependencies;
        deletable_cubes.len() as u32
    }

    fn get_total_destruction_fall_count(&self) -> u32 {
        let mut reversed_dependencies: HashMap<u32, Vec<u32>> = HashMap::new();
        for (id, dependencies) in &self.dependencies {
            for dependency in dependencies {
                reversed_dependencies
                    .entry(*dependency)
                    .or_insert_with(Vec::new)
                    .push(*id);
            }
        }

        let mut total = 0;
        for cube in &self.cubes {
            total += self.get_destruction_fall_count(cube.id, &reversed_dependencies);
        }
        total
    }

    fn get_destruction_fall_count(
        &self,
        id: u32,
        reversed_dependencies: &HashMap<u32, Vec<u32>>,
    ) -> u32 {
        let mut destroyed = HashSet::new();
        destroyed.insert(id);

        let mut destroy_queue = vec![id];

        while let Some(destroyed_id) = destroy_queue.pop() {
            if let Some(destroyed_with_id) = reversed_dependencies.get(&destroyed_id) {
                for id in destroyed_with_id {
                    if !destroyed.contains(id) {
                        if self
                            .dependencies
                            .get(id)
                            .unwrap()
                            .iter()
                            .all(|dependency_id| destroyed.contains(dependency_id))
                        {
                            destroyed.insert(*id);
                            destroy_queue.push(*id);
                        }
                    }
                }
            }
        }

        destroyed.len() as u32 - 1
    }
}

struct Cube {
    id: u32,
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

impl Cube {
    fn height(&self) -> u32 {
        self.z.end() - self.z.start() + 1
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        let mut parts = value.split("~");
        let mut start_parts = parts.next().unwrap().split(",");
        let x_start = start_parts.next().unwrap().parse::<u32>().unwrap();
        let y_start = start_parts.next().unwrap().parse::<u32>().unwrap();
        let z_start = start_parts.next().unwrap().parse::<u32>().unwrap();
        let mut end_parts = parts.next().unwrap().split(",");
        let x_end = end_parts.next().unwrap().parse::<u32>().unwrap();
        let y_end = end_parts.next().unwrap().parse::<u32>().unwrap();
        let z_end = end_parts.next().unwrap().parse::<u32>().unwrap();
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            x: x_start..=x_end,
            y: y_start..=y_end,
            z: z_start..=z_end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
