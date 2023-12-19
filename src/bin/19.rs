use std::collections::HashMap;
advent_of_code::solution!(19);

#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &'static str = "\n\n";

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, data) = parse_input(input);
    let mut sum = 0;
    for data in data {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let result = workflow.get_next_workflow(&data);

            if result == "A" {
                sum += data.sum();
                break;
            }
            if result == "R" {
                break;
            }

            workflow = workflows.get(result).unwrap();
        }

    }


    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (HashMap<&str, WorkFlow>, Vec<Data>) {
    let mut parts = input.split(EMPTY_LINE);
    let workflows = parse_workflows(parts.next().unwrap());
    let data = parse_data(parts.next().unwrap());
    (workflows, data)
}

fn parse_workflows<'a>(input: &'a str) -> HashMap<&'a str, WorkFlow> {
    let mut workflows = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split("{");
        let name = parts.next().unwrap();
        let data = parts.next().unwrap();
        let data = data.trim_end_matches("}");
        workflows.insert(name, data.into());
    }

    workflows
}

fn parse_data(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| line.into())
        .collect()
}

struct WorkFlow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> WorkFlow<'a> {
    fn get_next_workflow(&self, data: &Data) -> &'a str {
        for rule in &self.rules {
            if let Some(result) = rule.check(data) {
                return result;
            }
        }
        panic!("No rule found for data: {:?}", data);
    }
}

impl<'a> From<&'a str> for WorkFlow<'a> {
    fn from(value: &'a str) -> Self {
        let mut rules = Vec::new();
        for line in value.split(',') {
            rules.push(line.into());
        }
        WorkFlow { rules }
    }
}

struct Rule<'a> {
    check: Check,
    satisfy_value: SatisfyValue,
    satisfy_result: &'a str,
}

impl<'a> Rule<'a> {
    fn check(&self, data: &Data) -> Option<&'a str> {
        if self.satisfy_value.check(data, &self.check) {
            Some(self.satisfy_result)
        }
        else {
            None
        }

    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        if value.contains(':') {
            let mut outer_parts = value.split(":");
            let left = outer_parts.next().unwrap();
            let target = outer_parts.next().unwrap();

            if left.contains('<') {
                let mut inner_parts = left.split("<");
                let left = inner_parts.next().unwrap();
                let right = inner_parts.next().unwrap();
                Rule {
                    check: Check::LessThan,
                    satisfy_value: (left, right).into(),
                    satisfy_result: target,
                }
            }
            else {
                let mut inner_parts = left.split(">");
                let left = inner_parts.next().unwrap();
                let right = inner_parts.next().unwrap();
                Rule {
                    check: Check::GreaterThan,
                    satisfy_value: (left, right).into(),
                    satisfy_result: target,
                }
            }

        }
        else {
            Rule {
                check: Check::None,
                satisfy_value: SatisfyValue::None,
                satisfy_result: value,
            }
        }
    }
}

#[derive(Debug)]
struct Data {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        let data = &value[1..value.len() - 1];
        let mut parts = data.split(',');
        Data {
            x: Data::get_value(parts.next().unwrap()),
            m: Data::get_value(parts.next().unwrap()),
            a: Data::get_value(parts.next().unwrap()),
            s: Data::get_value(parts.next().unwrap()),
        }
    }


}

impl Data {
    fn get_value(value: &str) -> u32 {
        value.split('=').last().unwrap().parse().unwrap()
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

enum SatisfyValue {
    X(u32),
    M(u32),
    A(u32),
    S(u32),
    None,
}

impl SatisfyValue {
    fn check(&self, data: &Data, check: &Check) -> bool {
        match check {
            Check::GreaterThan => {
                match self {
                    SatisfyValue::X(value) => data.x > *value,
                    SatisfyValue::M(value) => data.m > *value,
                    SatisfyValue::A(value) => data.a > *value,
                    SatisfyValue::S(value) => data.s > *value,
                    SatisfyValue::None => true,
                }
            }
            Check::LessThan => {
                match self {
                    SatisfyValue::X(value) => data.x < *value,
                    SatisfyValue::M(value) => data.m < *value,
                    SatisfyValue::A(value) => data.a < *value,
                    SatisfyValue::S(value) => data.s < *value,
                    SatisfyValue::None => true,
                }
            },
            Check::None => true,
        }
    }
}

impl From<(&str, &str)> for SatisfyValue {
    fn from(value: (&str, &str)) -> Self {
        let identifier = value.0;
        let value = value.1.parse().unwrap();
        match identifier {
            "x" => SatisfyValue::X(value),
            "m" => SatisfyValue::M(value),
            "a" => SatisfyValue::A(value),
            "s" => SatisfyValue::S(value),
            _ => panic!("Unknown identifier"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Check {
    GreaterThan,
    LessThan,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
