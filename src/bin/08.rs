use num::integer::lcm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse_input(input);
    let start_node = Rc::clone(nodes.get("AAA").unwrap());

    Some(get_steps_until(start_node, instructions, "ZZZ"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse_input(input);

    let start_nodes: Vec<Rc<RefCell<Node>>> = nodes
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| Rc::clone(nodes.get(x).unwrap()))
        .collect();

    let lowest_common_multiple = start_nodes
        .iter()
        .map(|node| get_steps_until(Rc::clone(node), instructions, "Z"))
        .fold(1, |acc, x| lcm(acc, x));

    Some(lowest_common_multiple)
}

fn get_steps_until(node: Rc<RefCell<Node>>, instructions: &str, end: &str) -> u64 {
    let mut count = 0;
    let mut current_node = node;

    loop {
        for instruction in instructions.chars() {
            let new_node = current_node.borrow().get_for_instruction(instruction);
            match new_node {
                Some(node) => {
                    count += 1;
                    if node.borrow().name.ends_with(end) {
                        return count;
                    }
                    current_node = node
                }
                None => {
                    return 0;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (&str, HashMap<&str, Rc<RefCell<Node>>>) {
    let mut known_nodes: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    lines.next();

    for line in lines {
        parse_node(line, &mut known_nodes);
    }

    (instructions, known_nodes)
}

fn parse_node<'a>(
    line: &'a str,
    known_nodes: &mut HashMap<&'a str, Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    let mut parts = line.split(" = ");
    let node_name = parts.next().unwrap();
    let mut other_parts = parts.next().unwrap().split(", ");
    let left_name = &other_parts.next().unwrap()[1..];
    let right_name = &other_parts.next().unwrap()[..3];

    // get node from hashset or create new one
    let node = Rc::clone(
        known_nodes
            .entry(node_name)
            .or_insert(Rc::new(RefCell::new(Node::new(node_name)))),
    );
    let left = Rc::clone(
        known_nodes
            .entry(left_name)
            .or_insert(Rc::new(RefCell::new(Node::new(left_name)))),
    );
    let right = Rc::clone(
        known_nodes
            .entry(right_name)
            .or_insert(Rc::new(RefCell::new(Node::new(right_name)))),
    );

    let mut mut_node = node.borrow_mut();
    mut_node.left = Some(left);
    mut_node.right = Some(right);
    drop(mut_node);

    node
}

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        Self {
            name,
            left: None,
            right: None,
        }
    }

    fn get_for_instruction(&self, instruction: char) -> Option<Rc<RefCell<Node>>> {
        match instruction {
            'L' => self.left.clone(),
            'R' => self.right.clone(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_part2", DAY));
        assert_eq!(result, Some(6));
    }
}
