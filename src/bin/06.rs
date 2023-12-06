advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = read_input(input, &read_line_multi_races);

    let mut result = 1;
    for race in races {
        result *= race.count_valid_press_times();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = read_input(input, &read_line_single_race);

    let mut result = 1;
    for race in races {
        result *= race.count_valid_press_times();
    }

    Some(result)
}

fn read_input(input: &str, line_reader: &dyn Fn(&str) -> Vec<u64>) -> Vec<Race> {
    let mut lines = input.lines();
    let times = line_reader(lines.next().unwrap());
    let distances = line_reader(lines.next().unwrap());

    let mut races: Vec<Race> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        races.push(Race::new(*time, *distance));
    }
    races
}

fn read_line_multi_races(line: &str) -> Vec<u64> {
    let mut line_parts = line.split(':');
    line_parts.next();
    line_parts.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn read_line_single_race(line: &str) -> Vec<u64> {
    let mut line_parts = line.split(':');
    line_parts.next();
    let cleared = line_parts.next().unwrap().replace(" ", "");
    Vec::from([cleared.parse::<u64>().unwrap()])
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn count_valid_press_times(&self) -> u64 {
        // x^2 - x * time + distance < 0
        let time = self.time as f64;
        let distance = self.distance as f64;
        let determinant = (time * time - 4.0 * distance).sqrt();

        let x1 = (time - determinant) / 2.0;
        let x2 = (time + determinant) / 2.0;

        let used_x1 = x1.ceil() as u64 + if x1.fract() == 0.0 { 1 } else { 0 };
        let used_x2 = x2.floor() as u64 - if x2.fract() == 0.0 { 1 } else { 0 };
        
        used_x2 - used_x1 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
