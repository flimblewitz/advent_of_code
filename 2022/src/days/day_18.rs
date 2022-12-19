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

fn does_not_exist_to_usize(cube: (i16, i16, i16), cubes: &HashSet<(i16, i16, i16)>) -> usize {
    if cubes.contains(&cube) {
        0
    } else {
        1
    }
}

pub fn part_one(input: &str) -> usize {
    let cubes = parse_input(input);
    // println!("{:?}", cubes);
    cubes
        .iter()
        .map(|cube| {
            get_adjacent_cubes(*cube)
                .iter()
                .map(|ac| does_not_exist_to_usize(*ac, &cubes))
                .sum::<usize>()
        })
        .sum()
}

fn get_adjacent_cubes(cube: (i16, i16, i16)) -> [(i16, i16, i16); 6] {
    [
        (cube.0 - 1, cube.1, cube.2),
        (cube.0 + 1, cube.1, cube.2),
        (cube.0, cube.1 - 1, cube.2),
        (cube.0, cube.1 + 1, cube.2),
        (cube.0, cube.1, cube.2 - 1),
        (cube.0, cube.1, cube.2 + 1),
    ]
}

fn _does_not_exist_and_is_not_surrounded_to_usize(
    cube: (i16, i16, i16),
    cubes: &HashSet<(i16, i16, i16)>,
) -> usize {
    let adjacent_cubes = get_adjacent_cubes(cube);

    let is_surrounded_by_existing_cubes = adjacent_cubes
        .iter()
        .all(|adjacent_cube| cubes.contains(&adjacent_cube));

    // if is_surrounded_by_existing_cubes {
    //     println!(
    //         "{} surrounded by {}",
    //         format!("({},{},{})", cube.0, cube.1, cube.2),
    //         adjacent_cubes
    //             .map(|ac| format!("({},{},{})", ac.0, ac.1, ac.2))
    //             .join(", ")
    //     );
    // }

    if cubes.contains(&cube) || is_surrounded_by_existing_cubes {
        0
    } else {
        1
    }
}

// wrong: 4338 (high)
pub fn part_two(_input: &str) -> usize {
    // let cubes = parse_input(input);
    // println!("{:?}", cubes);

    // we are counting the number of nonexistent or non surrounded cubes
    // cubes
    //     .iter()
    //     .map(|cube| {
    //         get_adjacent_cubes(*cube)
    //             .iter()
    //             .map(|ac| _does_not_exist_and_is_not_surrounded_to_usize(*ac, &cubes))
    //             .sum::<usize>()
    //     })
    //     .sum()

    todo!("I need to consider pockets that are bigger than just the size of one cube")
}
