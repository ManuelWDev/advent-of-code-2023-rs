advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split(',').map(|part| custom_hash(part)).sum();

    Some(result)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: [Vec<Lens>; 256] = core::array::from_fn(|_| Vec::new());
    for instruction in input.split(',') {
        run_instruction(instruction, &mut boxes);
    }

    Some(calculate_focal_power(&boxes))
}

fn custom_hash(data: &str) -> u32 {
    let mut result = 0;
    for c in data.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    result
}

struct Lens<'a> {
    label: &'a str,
    number: u8,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, number: u8) -> Self {
        Self { label, number }
    }
}

fn run_instruction<'a>(instruction: &'a str, boxes: &mut [Vec<Lens<'a>>; 256]) {
    if instruction.ends_with('-') {
        let label = &instruction[..instruction.len() - 1];
        let used_box = &mut boxes[custom_hash(label) as usize];
        if let Some(remove_index) = used_box.iter().position(|lens| &lens.label == &label) {
            used_box.remove(remove_index);
        }
    } else {
        let mut parts = instruction.split('=');
        let label = parts.next().unwrap();
        let lens_number = parts.next().unwrap().parse::<u8>().unwrap();
        let used_box = &mut boxes[custom_hash(label) as usize];

        if let Some(label_index) = used_box.iter().position(|lens| &lens.label == &label) {
            used_box.remove(label_index);
            used_box.insert(label_index, Lens::new(label, lens_number));
        } else {
            used_box.push(Lens::new(label, lens_number));
        }
    }
}

fn calculate_focal_power(boxes: &[Vec<Lens>; 256]) -> u32 {
    let mut result = 0;
    for (i, used_box) in boxes.iter().enumerate() {
        let box_identifier = i + 1;
        for (j, lens) in used_box.iter().enumerate() {
            result += box_identifier * (j + 1) * (lens.number as usize);
        }
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
