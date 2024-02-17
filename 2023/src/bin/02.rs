advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // The Elf would first like to know which games would have been possible if the bag contained *only 12 red cubes, 13 green cubes, and 14 blue cubes*?
    Some(
        input
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                line.split_once(": ").unwrap().1.split("; ").all(|set| {
                    set.split(", ").all(|cubes_of_one_color| {
                        let (number, color) = cubes_of_one_color.split_once(" ").unwrap();
                        let number: u32 = number.parse().unwrap();
                        match color {
                            "red" => number <= 12,
                            "green" => number <= 13,
                            "blue" => number <= 14,
                            _ => panic!("unexpected color {color}"),
                        }
                    })
                })
            })
            // now we derive the game "id"s
            .map(|(line_index, _)| line_index as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let power = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    // for every set
                    .fold((0_u32, 0_u32, 0_u32), |mut acc, set| {
                        set.split(", ").for_each(|cubes_of_one_color| {
                            let (number, color) = cubes_of_one_color.split_once(" ").unwrap();
                            let number: u32 = number.parse().unwrap();
                            match color {
                                "red" => acc.0 = acc.0.max(number),
                                "green" => acc.1 = acc.1.max(number),
                                "blue" => acc.2 = acc.2.max(number),
                                _ => panic!("unexpected color {color}"),
                            }
                        });
                        acc
                    });
                power.0 * power.1 * power.2
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
