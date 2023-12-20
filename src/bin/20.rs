use std::collections::HashMap;
advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    let mut system = System::from(input);
    for _ in 0..1000 {
        system.push_button();
    }
    Some(system.low_pulses_sent * system.high_pulses_sent)
}

pub fn part_two(_: &str) -> Option<u32> {
    // part 2 is solved by hand
    None
}

struct System {
    modules: HashMap<String, Box<dyn Module>>,
    low_pulses_sent: u64,
    high_pulses_sent: u64,
}

impl System {
    fn push_button(&mut self) {
        let mut current_modules = vec![("broadcaster".to_string(), false, "".to_string())];
        self.low_pulses_sent += 1;
        while current_modules.len() > 0 {
            let current_module = current_modules.pop().unwrap();
            if let Some(module) = self.modules.get_mut(&current_module.0) {
                let (targets, high) = module.execute(current_module.1, &current_module.2);
                for target in targets.iter() {
                    current_modules.insert(0, (target.clone(), high, current_module.0.clone()));
                }
                if high {
                    self.high_pulses_sent += targets.len() as u64;
                } else {
                    self.low_pulses_sent += targets.len() as u64;
                }
            }
        }
    }
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut name_to_targets: HashMap<String, Vec<String>> = HashMap::new();
        for line in value.lines() {
            let mut parts = line.split(" -> ");
            let name = parts.next().unwrap();
            let targets = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|name| name.into())
                .collect::<Vec<_>>();
            if name.starts_with('%') {
                let name = name[1..].to_string();
                name_to_targets.insert(name.clone(), targets.clone());
                let module = Box::new(FlipFlop::new(targets));
                modules.insert(name, module);
            } else if name.starts_with('&') {
                let name = name[1..].to_string();
                name_to_targets.insert(name.clone(), targets.clone());
                modules.insert(name, Box::new(Conjunction::new(targets)));
            } else {
                let name = name.to_string();
                name_to_targets.insert(name.clone(), targets.clone());
                modules.insert(name, Box::new(Broadcast::new(targets)));
            }
        }

        // setup inputs for each module
        for (name, targets) in name_to_targets.iter() {
            for target in targets.iter() {
                if let Some(module) = modules.get_mut(target) {
                    module.set_input(name);
                }
            }
        }

        Self {
            modules,
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }
}

trait Module {
    fn execute(&mut self, high: bool, from: &str) -> (Vec<String>, bool);
    fn get_targets(&self) -> &Vec<String>;
    fn set_input(&mut self, inputs: &String);
}

struct FlipFlop {
    high: bool,
    targets: Vec<String>,
}

impl Module for FlipFlop {
    fn execute(&mut self, high: bool, _: &str) -> (Vec<String>, bool) {
        if high {
            (Vec::new(), false)
        } else {
            self.high = !self.high;
            (self.targets.clone(), self.high)
        }
    }

    fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn set_input(&mut self, _: &String) {}
}

impl FlipFlop {
    fn new(targets: Vec<String>) -> Self {
        Self {
            high: false,
            targets,
        }
    }
}

struct Conjunction {
    last_pulses: HashMap<String, bool>,
    targets: Vec<String>,
}

impl Module for Conjunction {
    fn execute(&mut self, high: bool, from: &str) -> (Vec<String>, bool) {
        self.last_pulses.insert(from.to_string(), high);
        let result = !self.last_pulses.values().all(|&x| x);
        (self.targets.clone(), result)
    }

    fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn set_input(&mut self, input: &String) {
        self.last_pulses.insert(input.clone(), false);
    }
}

impl Conjunction {
    fn new(targets: Vec<String>) -> Self {
        Self {
            last_pulses: HashMap::new(),
            targets,
        }
    }
}

struct Broadcast {
    targets: Vec<String>,
}

impl Module for Broadcast {
    fn execute(&mut self, high: bool, _: &str) -> (Vec<String>, bool) {
        (self.targets.clone(), high)
    }

    fn get_targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn set_input(&mut self, _: &String) {}
}

impl Broadcast {
    fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_second() {
        let result = part_one(&advent_of_code::template::read_file("examples_part2", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
