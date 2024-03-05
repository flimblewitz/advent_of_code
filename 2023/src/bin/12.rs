use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}
impl Condition {
    fn new(c: char) -> Self {
        match c {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("unexpected char {}", c),
        }
    }
}

#[derive(Debug)]
struct Row {
    records: Vec<Condition>,
    contiguous_damaged_group_lengths: Vec<usize>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<Row> = input
        .lines()
        .map(|row| {
            let (records, contiguous_damaged_groups): (&str, &str) = row.split_once(' ').unwrap();
            let records: Vec<Condition> = records.chars().map(|c| Condition::new(c)).collect();
            let contiguous_damaged_group_lengths: Vec<usize> = contiguous_damaged_groups
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            Row {
                records,
                contiguous_damaged_group_lengths,
            }
        })
        // .inspect(|row| println!("{:?}", row))
        .collect();

    let answer = rows
        .into_iter()
        .map(
            |Row {
                 records,
                 contiguous_damaged_group_lengths,
             }| {
                // println!("given records {:?}", records);
                // println!(
                //     "given contiguous_damaged_group_lengths {:?}",
                //     contiguous_damaged_group_lengths
                // );

                let mut theoretical_arrangements_by_length: HashMap<usize, Vec<Vec<Condition>>> =
                    HashMap::new();

                // todo!("use memoization!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

                let theoretical_arrangements = generate_theoretical_arrangements(
                    &records,
                    &mut theoretical_arrangements_by_length,
                );
                // println!(
                //     "theoretical_arrangements: {:?}",
                //     theoretical_arrangements
                // );
                // println!(
                //     "theoretical_arrangements len: {:?}",
                //     theoretical_arrangements.len()
                // );
                let count = theoretical_arrangements
                    .into_iter()
                    .filter(|records| {
                        let group_lengths: Vec<usize> = records
                            .split(|condition| match condition {
                                Condition::Operational => true,
                                _ => false,
                            })
                            .filter(|group| !group.is_empty())
                            .map(|group| group.len())
                            .collect();
                        let is_viable = group_lengths.len()
                            == contiguous_damaged_group_lengths.len()
                            && group_lengths
                                .into_iter()
                                .zip(contiguous_damaged_group_lengths.iter())
                                .all(|(a, b)| a == *b);
                        // if is_viable {
                        //     println!("viable: {:?}", records);
                        // }
                        is_viable
                    })
                    .count();
                // println!("count {}", count);
                count
            },
        )
        .sum::<usize>() as u32;

    Some(answer)
}

fn generate_theoretical_arrangements(
    records: &[Condition],
    theoretical_arrangements_by_length: &mut HashMap<usize, Vec<Vec<Condition>>>,
) -> Vec<Vec<Condition>> {
    if let Some(arrangements) = theoretical_arrangements_by_length.get(&records.len()) {
        return arrangements.clone();
    }
    let arrangements = match records.first() {
        Some(condition) => match condition {
            Condition::Unknown => {
                let theoretical_arrangements_given_damaged =
                    prepend_condition_to_subsequent_theoretical_arrangements(
                        records,
                        Condition::Damaged,
                        theoretical_arrangements_by_length,
                    );
                let theoretical_arrangements_given_operational =
                    prepend_condition_to_subsequent_theoretical_arrangements(
                        records,
                        Condition::Operational,
                        theoretical_arrangements_by_length,
                    );
                [
                    theoretical_arrangements_given_damaged,
                    theoretical_arrangements_given_operational,
                ]
                .concat()
            }
            _ => prepend_condition_to_subsequent_theoretical_arrangements(
                records,
                *condition,
                theoretical_arrangements_by_length,
            ),
        },
        _ => vec![vec![]],
    };
    theoretical_arrangements_by_length.insert(records.len(), arrangements.clone());
    arrangements
}

fn prepend_condition_to_subsequent_theoretical_arrangements(
    records: &[Condition],
    condition: Condition,
    theoretical_arrangements_by_length: &mut HashMap<usize, Vec<Vec<Condition>>>,
) -> Vec<Vec<Condition>> {
    generate_theoretical_arrangements(&records[1..], theoretical_arrangements_by_length)
        .into_iter()
        .map(|mut tail| {
            tail.insert(0, condition);
            tail
        })
        .collect()
}

// the naive approach for part one that considers all possible arrangements just doesn't cut it for part two. There are way too many possible arrangements to reasonably check all of them
// dynamic programming is once again the way to go
pub fn part_two(input: &str) -> Option<usize> {
    let answer = input
        .lines()
        .map(|row| {
            let (records, contiguous_damaged_group_lengths): (&str, &str) =
                row.split_once(' ').unwrap();

            let number_of_times_to_repeat = 5;

            let records = (0..number_of_times_to_repeat)
                .map(|_| records.to_owned())
                .collect::<Vec<String>>()
                .join("?");
            // putting a terminating period at the end makes this a lot easier
            // I don't think it's necessary, but I couldn't get it to work by using corresponding logic and not using one, so this is how it is
            let mut records = records.clone();
            records.push('.');

            let records: Vec<Condition> = records
                .chars()
                // .inspect(|c| print!("{}", c))
                .map(|c: char| Condition::new(c))
                .collect();
            // println!();

            // println!("given records {:?}", records);
            // records.iter().for_each(|condition| {
            //     let c = match condition {
            //         Condition::Operational => '.',
            //         Condition::Damaged => '#',
            //         Condition::Unknown => '?',
            //     };
            //     print!("{}", c);
            // });
            // println!();

            let damaged_group_lengths: Vec<usize> = contiguous_damaged_group_lengths
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
                .repeat(number_of_times_to_repeat);

            // println!(
            //     "{}",
            //     damaged_group_lengths
            //         .iter()
            //         .map(|l| l.to_string())
            //         .collect::<Vec<String>>()
            //         .join(",")
            // );

            // // this is the number of unknowns that are actually damages
            // let hidden_damage_count: usize = damaged_group_lengths.iter().sum::<usize>()
            //     - records.chars().filter(|c| *c == '#').count();

            // it's dynamic programming time, baby
            // the problem will be "given I'm at condition index X (and all preceding conditions are "certain"), where I'm currently forming a damaged group of length Y, and I've already finished Z damaged groups, how many possible arrangements can I form by deciding values for the unknown conditions at and after X?"
            // this is a total mouthful, but it'll save effort because you can basically end up in situations where you'll have ABC and AB'C, where B and B' are different subarrangements that still have the same number of finished damaged groups and the "current" damaged group at the end of B and B' is the same (for instancee, B could be ".#.#.#" and B' could be "#.#..#"). That following C can be a HUGE tree of possibilities that would be enormously wasteful to redundantly reconsider in the wake of B' after you've already considered it in the wake of B
            let mut subsolutions: HashMap<(usize, usize, usize), usize> = HashMap::new();

            let count = recursively_count_possible_arrangements(
                &mut subsolutions,
                &records,
                &damaged_group_lengths,
                0,
                0,
                0,
            );

            // println!("count: {}", count);

            count
        })
        .sum::<usize>();

    Some(answer)
}

fn recursively_count_possible_arrangements(
    subsolutions: &mut HashMap<(usize, usize, usize), usize>,
    records: &[Condition],
    damaged_group_lengths: &[usize],
    condition_index: usize,
    current_damaged_group_length: usize,
    finished_damaged_groups_count: usize,
) -> usize {
    if let Some(subsolution) = subsolutions.get(&(
        condition_index,
        current_damaged_group_length,
        finished_damaged_groups_count,
    )) {
        return *subsolution;
    }

    // since we don't have a subsolution yet, we need to calculate it

    // first, let's confirm we haven't stumbled into an invalid arrangement
    // if there are no more damaged groups to consider, we may be at the end of our rope
    let no_more_damaged_groups = || finished_damaged_groups_count == damaged_group_lengths.len();

    let current_damaged_group_finished =
        || current_damaged_group_length == damaged_group_lengths[finished_damaged_groups_count];

    // if we've just passed the end of the records, we need to stop and determine if we've ended up with a valid arrangement
    let count = if condition_index == records.len() {
        if no_more_damaged_groups() {
            // let's trust our process to not have created damaged groups with invalid lengths
            1
        } else {
            0
        }
    } else {
        // since we haven't yet reached the end of the records, possibilities may yet await us

        // TODO factor out the duplicated behavior from Damaged and Unknown and also Operational and Unknown respectively
        match records[condition_index] {
            Condition::Damaged => recursively_count_possible_arrangements(
                subsolutions,
                records,
                damaged_group_lengths,
                condition_index + 1,
                current_damaged_group_length + 1,
                finished_damaged_groups_count,
            ),
            Condition::Operational => {
                // either we have a current damaged group that we're "forming" or not
                if !no_more_damaged_groups() && current_damaged_group_finished() {
                    // if we just correctly finished the current damaged group, great
                    recursively_count_possible_arrangements(
                        subsolutions,
                        records,
                        damaged_group_lengths,
                        condition_index + 1,
                        0,
                        finished_damaged_groups_count + 1,
                    )
                } else if current_damaged_group_length == 0 {
                    // if we haven't formed a damaged group yet, then let's just keep moving
                    recursively_count_possible_arrangements(
                        subsolutions,
                        records,
                        damaged_group_lengths,
                        condition_index + 1,
                        0,
                        finished_damaged_groups_count,
                    )
                } else {
                    // otherwise we either have too many damaged groups or we're about to incorrectly finish a damaged group
                    0
                }
            }
            Condition::Unknown => {
                // TODO: let's prune this branch of possibilities if the current_damaged_group_length is already equal to the length of the corresponding official damaged group length

                let count_given_damaged = recursively_count_possible_arrangements(
                    subsolutions,
                    records,
                    damaged_group_lengths,
                    condition_index + 1,
                    current_damaged_group_length + 1,
                    finished_damaged_groups_count,
                );

                let count_given_operational =
                    if !no_more_damaged_groups() && current_damaged_group_finished() {
                        // if we just correctly finished the current damaged group, great
                        recursively_count_possible_arrangements(
                            subsolutions,
                            records,
                            damaged_group_lengths,
                            condition_index + 1,
                            0,
                            finished_damaged_groups_count + 1,
                        )
                    } else if current_damaged_group_length == 0 {
                        // if we haven't formed a damaged group yet, then let's just keep moving
                        recursively_count_possible_arrangements(
                            subsolutions,
                            records,
                            damaged_group_lengths,
                            condition_index + 1,
                            0,
                            finished_damaged_groups_count,
                        )
                    } else {
                        // otherwise we either have too many damaged groups or we're about to incorrectly finish a damaged group
                        0
                    };

                count_given_damaged + count_given_operational
            }
        }
    };

    subsolutions.insert(
        (
            condition_index,
            current_damaged_group_length,
            finished_damaged_groups_count,
        ),
        count,
    );

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
