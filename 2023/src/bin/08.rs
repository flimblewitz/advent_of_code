use core::panic;
use std::{collections::HashMap, mem::swap};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let lines = lines.skip(1);

    // todo let's try using &str after this
    let network: HashMap<String, (String, String)> = lines.fold(HashMap::new(), |mut acc, line| {
        let mut terms = line.split_terminator(" = ");

        let node = terms.next().unwrap();

        let mut neighbors = terms
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_terminator(", ");
        let left = neighbors.next().unwrap();
        let right = neighbors.next().unwrap();

        acc.insert(node.into(), (left.into(), right.into()));

        acc
    });

    let mut node = "AAA".into();
    // println!("{}", node);
    // println!("{:?}", network);
    let mut instructions = instructions.into_iter().cycle();
    let mut steps: u32 = 0;
    while node != "ZZZ" {
        steps += 1;
        match instructions.next() {
            Some('L') => node = network.get(&node).unwrap().0.to_owned(),
            Some('R') => node = network.get(&node).unwrap().1.to_owned(),
            _ => panic!("unexpected instruction"),
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let lines = lines.skip(1);

    // I'm using &str this time because I expect it to be faster
    let network: HashMap<&str, (&str, &str)> = lines.fold(HashMap::new(), |mut acc, line| {
        let mut terms = line.split_terminator(" = ");

        let node = terms.next().unwrap();

        let mut neighbors = terms
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_terminator(", ");
        let left = neighbors.next().unwrap();
        let right = neighbors.next().unwrap();

        acc.insert(node.into(), (left.into(), right.into()));

        acc
    });

    let starting_nodes: Vec<&str> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect();

    // println!("{:?}", nodes);
    // println!("{:?}", network);

    // the naive way to do this is to actually perform all of the traversals in parallel and wait for all of them to finally finish at ending points
    // but that may take a very long time. Let's assume that each starting node has a single ending node, because that means we can simply calculate how long the path is for each individual starting node and then find the least common multiple among those
    // for posterity, here's that naive solution
    // let mut instructions = instructions.into_iter().cycle();
    // let mut steps: u64 = 0;
    // while !starting_nodes.iter().all(|node| node.ends_with('Z')) {
    //     // println!("{:?}", nodes);
    //     let instruction = instructions.next().unwrap();
    //     // println!("{:?}", nodes);
    //     steps += 1;
    //     for node in starting_nodes.iter_mut() {
    //         let neighbors = network.get(node).unwrap();
    //         // println!("{} {} {:?}", node, instruction, neighbors);
    //         steps += 1;
    //         match instructions.next() {
    //             Some('L') => *node = neighbors.0,
    //             Some('R') => *node = neighbors.1,
    //             _ => panic!("unexpected instruction"),
    //         }
    //     }
    // }

    let steps_per_starting_node: Vec<u64> =
        starting_nodes.into_iter().fold(vec![], |acc, mut node| {
            let mut instructions = instructions.iter().cycle();
            let mut steps: u64 = 0;
            while !node.ends_with('Z') {
                steps += 1;
                let neighbors = network.get(node).unwrap();
                // println!("{} {} {:?}", node, instruction, neighbors);
                match instructions.next() {
                    Some('L') => node = neighbors.0,
                    Some('R') => node = neighbors.1,
                    _ => panic!("unexpected instruction"),
                }
            }
            [acc, vec![steps]].concat()
        });
    // println!("{:?}", steps_per_starting_node);

    let least_common_multiple = steps_per_starting_node
        .into_iter()
        .reduce(|acc, steps| least_common_multiple(acc, steps))
        .unwrap();

    Some(least_common_multiple)
}

fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    a * (b / greatest_common_divisor(a, b))
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
        let result = part_two(&advent_of_code::template::read_file_part(
            // "examples", DAY, 2,
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
