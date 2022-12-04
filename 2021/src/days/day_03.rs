use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let list_of_lists = parse_input(input);
    let length = list_of_lists.len();

    let mut index_sums = vec![0; list_of_lists.first().unwrap().len()];

    list_of_lists.iter().for_each(|digits| {
        digits
            .iter()
            .enumerate()
            .for_each(|(index, value)| index_sums[index] += value)
    });

    let discriminator = length / 2;

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    index_sums
        .iter()
        .rev()
        .enumerate()
        .for_each(|(index, value)| {
            if *value > discriminator as u32 {
                gamma_rate += u32::pow(2, index as u32);
            } else {
                epsilon_rate += u32::pow(2, index as u32);
            }
        });

    gamma_rate * epsilon_rate
}

pub fn part_two(input: &str) -> u32 {
    let list_of_lists = parse_input(input);

    let oxygen_generator_rating = get_rating(&list_of_lists, true);
    let co2_scrubber_rating = get_rating(&list_of_lists, false);

    oxygen_generator_rating * co2_scrubber_rating
}

fn get_rating(list_of_lists: &Vec<Vec<u32>>, use_most_common: bool) -> u32 {
    let inner_list_length = list_of_lists.first().unwrap().len();

    let mut desired_lists = list_of_lists.clone();

    for inner_list_index in 0..inner_list_length {
        if desired_lists.len() == 1 {
            break;
        }

        let mut indexes_of_inner_lists_with_zero_at_given_index = vec![];
        let mut indexes_of_inner_lists_with_one_at_given_index = vec![];

        desired_lists
            .iter()
            .enumerate()
            .for_each(|(outer_list_index, inner_list)| {
                if inner_list[inner_list_index] == 0 {
                    indexes_of_inner_lists_with_zero_at_given_index.push(outer_list_index)
                } else {
                    indexes_of_inner_lists_with_one_at_given_index.push(outer_list_index)
                }
            });

        let num_zeros_compared_to_ones = indexes_of_inner_lists_with_zero_at_given_index
            .len()
            .cmp(&indexes_of_inner_lists_with_one_at_given_index.len());

        let desired_indexes_of_inner_lists = match (num_zeros_compared_to_ones, use_most_common) {
            (Ordering::Less, true) => indexes_of_inner_lists_with_one_at_given_index,
            (Ordering::Less, false) => indexes_of_inner_lists_with_zero_at_given_index,
            (Ordering::Equal, true) => indexes_of_inner_lists_with_one_at_given_index,
            (Ordering::Equal, false) => indexes_of_inner_lists_with_zero_at_given_index,
            (Ordering::Greater, true) => indexes_of_inner_lists_with_zero_at_given_index,
            (Ordering::Greater, false) => indexes_of_inner_lists_with_one_at_given_index,
        };

        desired_lists = desired_indexes_of_inner_lists
            .iter()
            .map(|outer_list_index| desired_lists[*outer_list_index].clone())
            .collect();
    }

    desired_lists[0]
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, value)| {
            acc + value * u32::pow(2, index as u32)
        })
}

// let outer_indexes_to_keep = match (num_zeros_compared_to_ones, use_most_common) {
//     (Ordering::Less, true) => lists_with_one_at_given_index,
//     (Ordering::Less, false) => lists_with_zero_at_given_index,
//     (Ordering::Equal, true) => lists_with_one_at_given_index,
//     (Ordering::Equal, false) => lists_with_zero_at_given_index,
//     (Ordering::Greater, true) => lists_with_zero_at_given_index,
//     (Ordering::Greater, false) => lists_with_one_at_given_index,
// };

// let outer_indexes_to_keep = match lists_with_zero_at_given_index
//     .len()
//     .cmp(&lists_with_one_at_given_index.len())
// {
//     Ordering::Less => {
//         if use_most_common {
//             lists_with_one_at_given_index
//         } else {
//             lists_with_zero_at_given_index
//         }
//     }
//     Ordering::Equal => {
//         if use_most_common {
//             lists_with_one_at_given_index
//         } else {
//             lists_with_zero_at_given_index
//         }
//     }
//     Ordering::Greater => {
//         if use_most_common {
//             lists_with_zero_at_given_index
//         } else {
//             lists_with_one_at_given_index
//         }
//     }
// };

// let outer_indexes_to_keep =
//     if lists_with_zero_at_given_index.len() < lists_with_one_at_given_index.len() {
//         if use_most_common {
//             lists_with_zero_at_given_index
//         } else {
//             lists_with_one_at_given_index
//         }
//     } else if lists_with_zero_at_given_index.len() > lists_with_one_at_given_index.len() {
//         if use_most_common {
//             lists_with_one_at_given_index
//         } else {
//             lists_with_zero_at_given_index
//         }
//     } else {
//         if use_most_common {
//             lists_with_one_at_given_index
//         } else {
//             lists_with_zero_at_given_index
//         }
//     };

// let outer_indexes_to_keep = match (
//     lists_with_zero_at_given_index.len(),
//     lists_with_one_at_given_index.len(),
// ) {
//     (zeros, ones) if zeros > ones => {
//         if use_most_common {
//             lists_with_zero_at_given_index
//         } else {
//             lists_with_one_at_given_index
//         }
//     }
//     (zeros, ones) if zeros < ones => {
//         if use_most_common {
//             lists_with_one_at_given_index
//         } else {
//             lists_with_zero_at_given_index
//         }
//     }
//     _ if use_most_common => lists_with_one_at_given_index,
//     _ => lists_with_zero_at_given_index,
// };
