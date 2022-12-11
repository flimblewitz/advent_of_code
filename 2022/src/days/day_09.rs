use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(char, u32)> {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
            let distance: u32 = tokens[1].parse().unwrap();
            (tokens[0].chars().next().unwrap(), distance)
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let motions = parse_input(input);
    // println!("motions: {:?}", motions);
    let mut head = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    for (direction, distance) in motions {
        for _step in 1..=distance {
            match direction {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                c => panic!("unrecognized direction '{c}'"),
            };

            let x_diff = head.0 - tail.0;
            let y_diff = head.1 - tail.1;
            if x_diff.abs() > 1 || y_diff.abs() > 1 {
                tail.0 += x_diff.signum();
                tail.1 += y_diff.signum();
            }

            // println!("== {direction} {distance} ==");
            // println!("head: {:?}", head);
            // println!("tail: {:?}", tail);

            visited_positions.insert(tail);
        }
    }

    // let mut v: Vec<_> = visited_positions.iter().collect();
    // v.sort();
    // println!("{:?}", v);

    visited_positions.len()
}

pub fn part_two(input: &str) -> usize {
    let motions = parse_input(input);

    let mut knots = [(0, 0); 10];

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    for (direction, distance) in motions {
        for _step in 1..=distance {
            match direction {
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                c => panic!("unrecognized direction '{c}'"),
            };

            for i in 1..10 {
                let x_diff: i32 = knots[i - 1].0 - knots[i].0;
                let y_diff: i32 = knots[i - 1].1 - knots[i].1;
                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    knots[i].0 += x_diff.signum();
                    knots[i].1 += y_diff.signum();
                }
            }

            visited_positions.insert(*knots.last().unwrap());
        }
    }

    visited_positions.len()
}
