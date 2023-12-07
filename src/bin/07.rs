use std::cmp::Ordering;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse_input(input, 11);
    Some(get_value(&mut hands))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse_input(input, 0);
    Some(get_value(&mut hands))
}

fn parse_input(input: &str, joker_value: u8) -> Vec<Hand> {
    input.lines().map(|line| create_hand(line, joker_value)).collect()
}

fn get_value(hands: &mut Vec<Hand>) -> u32 {
    hands.sort();

    let mut result: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        result += (index as u32 + 1) * hand.bid;
    }
    result
}
#[derive(Eq)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
    hand_type: HandType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn create_hand(line: &str, joker_value: u8) -> Hand {
    let mut parts = line.split_whitespace();

    let cards = create_cards(parts.next().unwrap(), joker_value);
    let bid = parts.next().unwrap().parse::<u32>().unwrap();

    let hand_type = get_hand_type(cards);
    Hand { cards, bid, hand_type }
}

fn create_cards(card_string: &str, joker_value: u8) -> [u8; 5] {
    let mut cards = [0; 5];
    for (index, char) in card_string.chars().enumerate() {
        cards[index] = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => joker_value,
            'T' => 10,
            _ => char.to_digit(10).unwrap() as u8,
        };
    }
    cards
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match self_card.cmp(other_card) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

fn get_hand_type(cards: [u8; 5]) -> HandType {
    let mut counts = [0; 15];
    for card in cards.iter() {
        counts[*card as usize] += 1;
    }

    let mut types = [0; 5];

    // first count is the joker count
    for count in counts.iter().skip(1) {
        if *count == 0 {
            continue;
        }
        types[*count as usize - 1] += 1;
    }

    let jokers = counts[0];
    for _ in 0..jokers {
        let mut joker_used = false;
        for index in (0..types.len()).rev() {
            if types[index] > 0 {
                types[index] -= 1;
                types[index + 1] += 1;
                joker_used = true;
                break;
            }
        }

        if !joker_used {
            types[0] += 1;
        }
    }

    match types {
        [5, 0, 0, 0, 0] => HandType::HighCard,
        [3, 1, 0, 0, 0] => HandType::OnePair,
        [1, 2, 0, 0, 0] => HandType::TwoPairs,
        [2, 0, 1, 0, 0] => HandType::ThreeOfAKind,
        [0, 1, 1, 0, 0] => HandType::FullHouse,
        [1, 0, 0, 1, 0] => HandType::FourOfAKind,
        [0, 0, 0, 0, 1] => HandType::FiveOfAKind,
        _ => panic!("Invalid hand"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
