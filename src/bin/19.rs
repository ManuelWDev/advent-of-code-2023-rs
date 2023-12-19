use std::collections::HashMap;
use std::ops::Range;
advent_of_code::solution!(19);

#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const EMPTY_LINE: &'static str = "\n\n";

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, data) = parse_input(input);
    Some(calculate_sum(workflows, data, &|data| data.start_sum()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let workflows = parse_workflows(input.split(EMPTY_LINE).next().unwrap());
    let data = vec![Data {
        data: [
            1..4001, // x
            1..4001, // m
            1..4001, // a
            1..4001, // s
        ],
    }];
    Some(calculate_sum(workflows, data, &|data| data.field_count()))
}

fn calculate_sum(
    workflows: HashMap<&str, WorkFlow>,
    data: Vec<Data>,
    sum_function: &dyn Fn(&Data) -> u64,
) -> u64 {
    let mut sum = 0;
    for data in data {
        let mut current_datas = vec![(data, "in")];
        while current_datas.len() > 0 {
            let current_data = current_datas.pop().unwrap();
            let workflow = workflows.get(current_data.1).unwrap();
            let result = workflow.get_next_workflows(current_data.0);

            for (data, target) in result {
                if target == "A" {
                    sum += sum_function(&data);
                    continue;
                }
                if target == "R" {
                    continue;
                }

                current_datas.push((data, target));
            }
        }
    }
    sum
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
    input.lines().map(|line| line.into()).collect()
}

struct WorkFlow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> WorkFlow<'a> {
    fn get_next_workflows(&self, data: Data) -> Vec<(Data, &'a str)> {
        let mut output_data = Vec::new();
        let mut test_data = data;

        for rule in &self.rules {
            let (accepted, rejected) = rule.check(test_data);
            if let Some(accepted) = accepted {
                output_data.push(accepted);
            }
            if let Some(rejected) = rejected {
                test_data = rejected;
            } else {
                break;
            }
        }
        output_data
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
    fn check(&self, data: Data) -> (Option<(Data, &'a str)>, Option<Data>) {
        let result = self
            .satisfy_value
            .get_allowed_and_blocked(data, &self.check);
        let mut accepted = None;
        if result.0.is_some() {
            accepted = Some((result.0.unwrap(), self.satisfy_result));
        }
        (accepted, result.1)
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
            } else {
                let mut inner_parts = left.split(">");
                let left = inner_parts.next().unwrap();
                let right = inner_parts.next().unwrap();
                Rule {
                    check: Check::GreaterThan,
                    satisfy_value: (left, right).into(),
                    satisfy_result: target,
                }
            }
        } else {
            Rule {
                check: Check::None,
                satisfy_value: SatisfyValue::None,
                satisfy_result: value,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    data: [Range<u32>; 4],
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        let data = &value[1..value.len() - 1];
        let mut parts = data.split(',');
        let x = Data::get_value(parts.next().unwrap());
        let m = Data::get_value(parts.next().unwrap());
        let a = Data::get_value(parts.next().unwrap());
        let s = Data::get_value(parts.next().unwrap());
        Data {
            data: [x..(x + 1), m..(m + 1), a..(a + 1), s..(s + 1)],
        }
    }
}

impl Data {
    fn get_value(value: &str) -> u32 {
        value.split('=').last().unwrap().parse().unwrap()
    }

    fn field_count(&self) -> u64 {
        self.data.iter().map(|range| range.len() as u64).product()
    }

    fn start_sum(&self) -> u64 {
        self.data.iter().map(|range| range.start as u64).sum()
    }

    fn split_at(self, value: u32, index: usize) -> (Option<Data>, Option<Data>) {
        if self.data[index].end <= value {
            return (None, Some(self));
        }
        if self.data[index].start > value {
            return (Some(self), None);
        }

        let value = value + 1;

        let mut outside_data = self.data.clone();
        outside_data[index] = self.data[index].start..value;

        let outside = Data { data: outside_data };

        let mut inside_data = self.data;
        inside_data[index] = value..inside_data[index].end;

        let inside = Data { data: inside_data };
        (Some(inside), Some(outside))
    }

    fn split_at_x(self, value: u32) -> (Option<Data>, Option<Data>) {
        self.split_at(value, 0)
    }

    fn split_at_m(self, value: u32) -> (Option<Data>, Option<Data>) {
        self.split_at(value, 1)
    }

    fn split_at_a(self, value: u32) -> (Option<Data>, Option<Data>) {
        self.split_at(value, 2)
    }

    fn split_at_s(self, value: u32) -> (Option<Data>, Option<Data>) {
        self.split_at(value, 3)
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
    fn get_allowed_and_blocked(&self, data: Data, check: &Check) -> (Option<Data>, Option<Data>) {
        match check {
            Check::GreaterThan => match self {
                SatisfyValue::X(value) => data.split_at_x(*value),
                SatisfyValue::M(value) => data.split_at_m(*value),
                SatisfyValue::A(value) => data.split_at_a(*value),
                SatisfyValue::S(value) => data.split_at_s(*value),
                SatisfyValue::None => (Some(data), None),
            },
            Check::LessThan => match self {
                SatisfyValue::X(value) => {
                    let (a, b) = data.split_at_x(*value - 1);
                    (b, a)
                }
                SatisfyValue::M(value) => {
                    let (a, b) = data.split_at_m(*value - 1);
                    (b, a)
                }
                SatisfyValue::A(value) => {
                    let (a, b) = data.split_at_a(*value - 1);
                    (b, a)
                }
                SatisfyValue::S(value) => {
                    let (a, b) = data.split_at_s(*value - 1);
                    (b, a)
                }
                SatisfyValue::None => (Some(data), None),
            },
            Check::None => (Some(data), None),
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
        assert_eq!(result, Some(167409079868000));
    }
}
