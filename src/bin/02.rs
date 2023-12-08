advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let allowed_red = 12;
    let allowed_green = 13;
    let allowed_blue = 14;

    let mut allowed_id_sum = 0;

    for line in input.lines() {
        let game = Game::from(line);
        if game.max_red() <= allowed_red
            && game.max_green() <= allowed_green
            && game.max_blue() <= allowed_blue
        {
            allowed_id_sum += game.id;
        }
    }

    Some(allowed_id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cube_power_sum = 0;
    for line in input.lines() {
        let game = Game::from(line);
        cube_power_sum += game.max_red() * game.max_green() * game.max_blue();
    }

    Some(cube_power_sum)
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Pull {
    fn from(value: &str) -> Self {
        let mut colors = value.split(',');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        while let Some(color) = colors.next() {
            let mut color_infos = color.trim().split(' ');
            let color_count = color_infos.next().unwrap().parse::<u32>().unwrap();
            let color_name = color_infos.next().unwrap();

            match color_name {
                "red" => red = color_count,
                "green" => green = color_count,
                "blue" => blue = color_count,
                _ => panic!("Unknown color: {}", color_name),
            }
        }

        Pull { red, green, blue }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut parts = value.split(": ");
        let mut game_parts = parts.next().unwrap().split(' ');
        game_parts.next();
        let game_id = game_parts
            .next()
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let pulls = parts
            .next()
            .unwrap()
            .split(";")
            .map(|pull| Pull::from(pull))
            .collect::<Vec<Pull>>();

        Game { id: game_id, pulls }
    }
}

impl Game {
    fn max_red(&self) -> u32 {
        self.pulls.iter().map(|pull| pull.red).max().unwrap()
    }

    fn max_green(&self) -> u32 {
        self.pulls.iter().map(|pull| pull.green).max().unwrap()
    }

    fn max_blue(&self) -> u32 {
        self.pulls.iter().map(|pull| pull.blue).max().unwrap()
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
