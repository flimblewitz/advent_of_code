pub fn part_one(input: &str) -> usize {
    let chars_in_both_compartments_per_rucksack = input
        .lines()
        .map(|line| {
            let compartments = line.split_at(line.len() / 2);
            compartments
                .0
                .chars()
                .find(|c| compartments.1.contains(*c))
                .unwrap()
        })
        .collect();
    get_sum(chars_in_both_compartments_per_rucksack)
}

pub fn part_two(input: &str) -> usize {
    let rucksacks: Vec<&str> = input.lines().collect();
    let chars_common_in_each_three_rucksacks = rucksacks
        .chunks(3)
        .map(|three_rucksacks| {
            three_rucksacks[0]
                .chars()
                .find(|c| three_rucksacks[1].contains(*c) && three_rucksacks[2].contains(*c))
                .unwrap()
        })
        .collect();
    get_sum(chars_common_in_each_three_rucksacks)
}

fn get_sum(chars: Vec<char>) -> usize {
    chars
        .iter()
        .map(|c| match *c as usize {
            ascii_value if ascii_value >= 65 && ascii_value <= 90 => ascii_value - 65 + 27,
            ascii_value if ascii_value >= 97 && ascii_value <= 122 => ascii_value - 97 + 1,
            _ => panic!("jinkies"),
        })
        .sum()
}
