use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<(i16, i16, i16)> {
    input.lines().fold(HashSet::new(), |mut acc, line| {
        let mut vals = line.split(",").map(|val| val.parse::<i16>().unwrap());
        acc.insert((
            vals.next().unwrap(),
            vals.next().unwrap(),
            vals.next().unwrap(),
        ));
        acc
    })
}

fn does_not_exist_to_u16(cube: (i16, i16, i16), cubes: &HashSet<(i16, i16, i16)>) -> usize {
    if cubes.contains(&cube) {
        0
    } else {
        1
    }
}

pub fn part_one(input: &str) -> usize {
    let cubes = parse_input(input);
    println!("{:?}", cubes);
    cubes
        .iter()
        .map(|cube| {
            does_not_exist_to_u16((cube.0 - 1, cube.1, cube.2), &cubes)
                + does_not_exist_to_u16((cube.0 + 1, cube.1, cube.2), &cubes)
                + does_not_exist_to_u16((cube.0, cube.1 - 1, cube.2), &cubes)
                + does_not_exist_to_u16((cube.0, cube.1 + 1, cube.2), &cubes)
                + does_not_exist_to_u16((cube.0, cube.1, cube.2 - 1), &cubes)
                + does_not_exist_to_u16((cube.0, cube.1, cube.2 + 1), &cubes)
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    todo!()
}
