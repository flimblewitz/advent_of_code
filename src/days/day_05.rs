use std::collections::HashMap;

// #[derive(Debug)]
type LineSegment = ((u32, u32), (u32, u32));

fn parse_string_tuple(string_tuple: &str) -> (u32, u32) {
    string_tuple
        .split_once(",")
        .map(|x_y| (x_y.0.parse::<u32>().unwrap(), x_y.1.parse::<u32>().unwrap()))
        .unwrap()
}
fn parse_input(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .map(|start_end| {
                    (
                        parse_string_tuple(start_end.0),
                        parse_string_tuple(start_end.1),
                    )
                })
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let line_segments = parse_input(input);
    //println!("{:?}", line_segments);
    let mut intersection_count = 0;
    let mut point_weights = HashMap::new();
    line_segments
        .iter()
        .for_each(|((start_x, start_y), (end_x, end_y))| {
            //println!("{:?}", ((start_x, start_y), (end_x, end_y)));
            if *start_x == *end_x {
                let (lesser, greater) = match *start_y <= *end_y {
                    true => (*start_y, *end_y),
                    false => (*end_y, *start_y),
                };
                (lesser..=greater).for_each(|y| {
                    //println!("{}, {}", *start_x, y);
                    let weight = point_weights
                        .entry((*start_x, y))
                        .and_modify(|weight| *weight += 1)
                        .or_insert(1_u32);
                    if *weight == 2 {
                        //println!("bam");
                        intersection_count += 1;
                    }
                });
            } else if *start_y == *end_y {
                let (lesser, greater) = match *start_x <= *end_x {
                    true => (*start_x, *end_x),
                    false => (*end_x, *start_x),
                };
                (lesser..=greater).for_each(|x| {
                    //println!("{}, {}", x, *start_y);
                    let weight = point_weights
                        .entry((x, *start_y))
                        .and_modify(|weight| *weight += 1)
                        .or_insert(1_u32);
                    if *weight == 2 {
                        //println!("bam");
                        intersection_count += 1;
                    }
                });
            } else {
                //println!("skipping");
            }
        });

    intersection_count
}

pub fn part_two(input: &str) -> u32 {
    let line_segments = parse_input(input);
    //println!("{:?}", line_segments);
    let mut intersection_count = 0;
    let mut point_weights = HashMap::new();
    line_segments
        .iter()
        .for_each(|((start_x, start_y), (end_x, end_y))| {
            // println!("{:?}", ((start_x, start_y), (end_x, end_y)));
            let (lesser_x, greater_x) = match *start_x <= *end_x {
                true => (*start_x, *end_x),
                false => (*end_x, *start_x),
            };
            let (lesser_y, greater_y) = match *start_y <= *end_y {
                true => (*start_y, *end_y),
                false => (*end_y, *start_y),
            };
            if *start_x == *end_x {
                (lesser_y..=greater_y).for_each(|y| {
                    // println!("{}, {}", *start_x, y);
                    let weight = point_weights
                        .entry((*start_x, y))
                        .and_modify(|weight| *weight += 1)
                        .or_insert(1_u32);
                    if *weight == 2 {
                        // println!("bam");
                        intersection_count += 1;
                    }
                });
            } else if *start_y == *end_y {
                (lesser_x..=greater_x).for_each(|x| {
                    // println!("{}, {}", x, *start_y);
                    let weight = point_weights
                        .entry((x, *start_y))
                        .and_modify(|weight| *weight += 1)
                        .or_insert(1_u32);
                    if *weight == 2 {
                        // println!("bam");
                        intersection_count += 1;
                    }
                });
            } else {
                let x_iter = lesser_x..=greater_x;
                let x_vec: Vec<u32> = if start_x > end_x {
                    x_iter.collect()
                } else {
                    x_iter.rev().collect()
                };
                let y_iter = lesser_y..=greater_y;
                let y_vec: Vec<u32> = if start_y > end_y {
                    y_iter.collect()
                } else {
                    y_iter.rev().collect()
                };
                x_vec.iter().zip(y_vec.iter()).for_each(|(x, y)| {
                    // println!("{}, {}", x, y);
                    let weight = point_weights
                        .entry((*x, *y))
                        .and_modify(|weight| *weight += 1)
                        .or_insert(1_u32);
                    if *weight == 2 {
                        // println!("bam");
                        intersection_count += 1;
                    }
                });
            }
        });

    intersection_count
}
