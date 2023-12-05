advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappers) = parse_input(input, &parse_seeds_line);

    let mut smallest_location = u64::MAX;
    for seed in seeds {
        let mut current_state = seed;
        for mapper in &mappers {
            current_state = mapper.map(current_state);
        }
        if current_state < smallest_location {
            smallest_location = current_state;
        }
    }
    Some(smallest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seed_ranges, mappers) = parse_input(input, &parse_seed_ranges);

    let mut smallest_location = u64::MAX;
    for seed_range in seed_ranges {
        // this is the brute force approach, but it's fast enough
        // we could be working with ranges instead of individual seeds
        for seed in seed_range.start..seed_range.end {
            let mut current_state = seed;
            for mapper in &mappers {
                current_state = mapper.map(current_state);
            }
            if current_state < smallest_location {
                smallest_location = current_state;
            }
        }
    }

    Some(smallest_location)
}

struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl RangeMap {
    fn is_in_seed(&self, index: u64) -> bool {
        index >= self.source_start && index < self.source_start + self.length
    }

    fn seed_to_soil(&self, index: u64) -> u64 {
        if self.is_in_seed(index) {
            self.destination_start + (index - self.source_start)
        } else {
            index
        }
    }
}

impl From<&str> for RangeMap {
    fn from(input: &str) -> Self {
        let mut parts = input.split(" ");
        let destination_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let length = parts.next().unwrap().parse::<u64>().unwrap();
        Self {
            source_start,
            destination_start,
            length,
        }
    }
}

struct Mapper {
    maps: Vec<RangeMap>,
}

impl From<&str> for Mapper {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        lines.next();
        let maps = lines.map(|line| RangeMap::from(line)).collect();
        Self { maps }
    }
}

impl Mapper {
    fn map(&self, value: u64) -> u64 {
        let mut result = value;
        for map in &self.maps {
            result = map.seed_to_soil(result);
            if result != value {
                break;
            }
        }
        result
    }
}

fn parse_input<T>(input: &str, seed_parser: &dyn Fn(&str) -> T) -> (T, Vec<Mapper>) {
    let mut newline_parts = input.split("\r\n\r\n").skip_while(|line| line.is_empty());
    let seeds = seed_parser(newline_parts.next().unwrap());
    let mut mappers = Vec::new();
    for line in newline_parts {
        mappers.push(Mapper::from(line));
    }
    (seeds, mappers)
}

fn parse_seeds_line(line: &str) -> Vec<u64> {
    let mut parts = line.split(": ");
    parts.next();
    let seeds_text = parts.next().unwrap();
    let mut seeds = Vec::new();
    for seed in seeds_text.split_whitespace() {
        seeds.push(seed.parse::<u64>().unwrap());
    }
    seeds
}

fn parse_seed_ranges(line: &str) -> Vec<Range> {
    let mut parts = line.split(": ");
    parts.next();
    let seeds_text = parts.next().unwrap();
    let mut seeds = Vec::new();
    let mut whitespace_parts = seeds_text.split_whitespace();
    while let (Some(start), Some(length)) = (whitespace_parts.next(), whitespace_parts.next()) {
        let start = start.parse::<u64>().unwrap();
        seeds.push(Range::new(
            start,
            start + length.parse::<u64>().unwrap(),
        ));
    }
    seeds
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
