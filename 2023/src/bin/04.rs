use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (winning_numbers, numbers_we_have) =
                    line.split(": ").last().unwrap().split_once(" | ").unwrap();
                let winning_numbers: HashSet<u32> = winning_numbers
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect();
                // println!("winning_numbers: {:?}", winning_numbers);
                let numbers_we_have: Vec<u32> = numbers_we_have
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect();
                // println!("numbers_we_have: {:?}", numbers_we_have);
                let count_of_winning_numbers_we_have = numbers_we_have
                    .iter()
                    .filter(|number| winning_numbers.contains(number))
                    .count();
                // println!(
                //     "count_of_winning_numbers_we_have: {:?}",
                //     count_of_winning_numbers_we_have
                // );
                if count_of_winning_numbers_we_have > 0 {
                    let card_score = 2u32.pow(count_of_winning_numbers_we_have as u32 - 1);
                    // println!("card_score: {}", card_score);
                    card_score
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // let mut card_counts = vec![1; input.lines().count()];
    let card_counts: Vec<u32> = input.lines().enumerate().fold(
        vec![1; input.lines().count()],
        |mut acc, (card_index, line)| {
            let (winning_numbers, numbers_we_have) =
                line.split(": ").last().unwrap().split_once(" | ").unwrap();
            let winning_numbers: HashSet<u32> = winning_numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            // println!("winning_numbers: {:?}", winning_numbers);
            let numbers_we_have: Vec<u32> = numbers_we_have
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            // println!("numbers_we_have: {:?}", numbers_we_have);
            let count_of_winning_numbers_we_have = numbers_we_have
                .iter()
                .filter(|number| winning_numbers.contains(number))
                .count();
            // println!(
            //     "count_of_winning_numbers_we_have: {:?}",
            //     count_of_winning_numbers_we_have
            // );
            for i in 0..count_of_winning_numbers_we_have {
                acc[card_index + i + 1] += acc[card_index];
            }
            // println!("acc: {:?}", acc);
            acc
        },
    );
    Some(card_counts.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
