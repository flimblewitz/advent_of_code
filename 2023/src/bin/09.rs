advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let values: Vec<i32> = line
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();

                let mut history: Vec<Vec<i32>> = vec![values];
                while !history.last().unwrap().iter().all(|v| *v == 0) {
                    history.push(
                        history
                            .last()
                            .unwrap()
                            .windows(2)
                            .map(|window| window[1] - window[0])
                            .collect(),
                    )
                }

                let next: i32 = history
                    .iter()
                    .rev()
                    .map(|values| values.last().unwrap())
                    .sum();

                next
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let values: Vec<i32> = line
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();

                let mut history: Vec<Vec<i32>> = vec![values];
                while !history.last().unwrap().iter().all(|v| *v == 0) {
                    history.push(
                        history
                            .last()
                            .unwrap()
                            .windows(2)
                            .map(|window| window[1] - window[0])
                            .collect(),
                    )
                }

                let previous: i32 = history
                    .iter()
                    .rev()
                    .map(|values| *values.first().unwrap())
                    .reduce(|acc, e| e - acc)
                    .unwrap();

                // println!("{}", previous);

                previous
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
