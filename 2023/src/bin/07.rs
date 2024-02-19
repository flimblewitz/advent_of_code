advent_of_code::solution!(7);

const PART_ONE_CARDS_ORDERED_BY_VALUE: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug)]
struct PartOneHand {
    bid: u32,
    cards: String, // &str might be noticeably more performant
    hand_type: u8,
}
impl PartOneHand {
    fn new(line: &str) -> Self {
        let mut terms = line.split_whitespace();
        let cards: String = terms.next().unwrap().into();
        let bid: u32 = terms.next().unwrap().parse().unwrap();
        // to identify the type, I need to get the number of occurrences of each card
        let mut card_counts = cards.chars().fold(
            vec![0u8; PART_ONE_CARDS_ORDERED_BY_VALUE.len()],
            |mut acc, card| {
                let index = PART_ONE_CARDS_ORDERED_BY_VALUE
                    .iter()
                    .position(|c| *c == card)
                    .unwrap();
                acc[index] += 1;
                acc
            },
        );
        card_counts.sort();
        // reversing the card_counts makes it easier to identify the hand_type
        card_counts.reverse();
        // println!("cards: {}, card_counts: {:?}", cards, card_counts);
        let hand_type = match card_counts[..] {
            [5, ..] => 1,
            [4, ..] => 2,
            [3, 2, ..] => 3,
            [3, ..] => 4,
            [2, 2, ..] => 5,
            [2, ..] => 6,
            _ => 7,
        };
        Self {
            bid,
            cards,
            hand_type,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<PartOneHand> = input.lines().map(|line| PartOneHand::new(line)).collect();
    // I could implement the Ord trait for Hand, but converting them to tuples is easier because tuples are automatically sorted lexicographically
    hands.sort_by_key(|hand| {
        (
            // we want to sort first by hand_type
            hand.hand_type,
            // tiebreakers are defined by the strongest first card
            hand.cards
                .chars()
                .map(|card| {
                    PART_ONE_CARDS_ORDERED_BY_VALUE
                        .iter()
                        .position(|c| *c == card)
                        .unwrap()
                })
                .collect::<Vec<usize>>(),
        )
    });
    // I've been sorting hands in the order of most valuable to least valuable
    // let's reverse the order to make it easier to derive the corresponding ranks
    hands.reverse();
    // hands.iter().for_each(|h| println!("{:?}", h));
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
}

const PART_TWO_CARDS_ORDERED_BY_VALUE: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug)]
struct PartTwoHand {
    bid: u32,
    cards: String, // &str might be noticeably more performant
    hand_type: u8,
}
impl PartTwoHand {
    fn new(line: &str) -> Self {
        let mut terms = line.split_whitespace();
        let cards: String = terms.next().unwrap().into();
        let bid: u32 = terms.next().unwrap().parse().unwrap();
        // to identify the type, I need to get the number of occurrences of each card
        let mut card_counts = cards.chars().fold(
            vec![0u8; PART_TWO_CARDS_ORDERED_BY_VALUE.len()],
            |mut acc, card| {
                let index = PART_TWO_CARDS_ORDERED_BY_VALUE
                    .iter()
                    .position(|c| *c == card)
                    .unwrap();
                acc[index] += 1;
                acc
            },
        );
        let joker_count = card_counts.pop().unwrap();
        card_counts.sort();
        // reversing the card_counts makes it easier to identify the hand_type
        card_counts.reverse();
        // println!(
        //     "cards: {}, card_counts: {:?}, joker_count: {}",
        //     cards, card_counts, joker_count
        // );
        let hand_type = match card_counts[..] {
            [x, ..] if x + joker_count == 5 => 1,
            [x, ..] if x + joker_count == 4 => 2,
            [x, 2, ..] if x + joker_count == 3 => 3,
            [x, ..] if x + joker_count == 3 => 4,
            [x, 2, ..] if x + joker_count == 2 => 5,
            [x, ..] if x + joker_count == 2 => 6,
            _ => 7,
        };
        Self {
            bid,
            cards,
            hand_type,
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<PartTwoHand> = input.lines().map(|line| PartTwoHand::new(line)).collect();
    // I could implement the Ord trait for Hand, but converting them to tuples is easier because tuples are automatically sorted lexicographically
    hands.sort_by_key(|hand| {
        (
            // we want to sort first by hand_type
            hand.hand_type,
            // tiebreakers are defined by the strongest first card
            hand.cards
                .chars()
                .map(|card| {
                    PART_TWO_CARDS_ORDERED_BY_VALUE
                        .iter()
                        .position(|c| *c == card)
                        .unwrap()
                })
                .collect::<Vec<usize>>(),
        )
    });
    // I've been sorting hands in the order of most valuable to least valuable
    // let's reverse the order to make it easier to derive the corresponding ranks
    hands.reverse();
    // hands.iter().for_each(|h| println!("{:?}", h));
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
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
