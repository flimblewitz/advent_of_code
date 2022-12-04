fn parse_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|assignment| assignment.split("-").map(|s| s.parse::<usize>().unwrap()))
                .map(|mut assigment_nums| {
                    (
                        assigment_nums.next().unwrap(),
                        assigment_nums.next().unwrap(),
                    )
                })
        })
        .map(|mut assigment_tuples| {
            (
                assigment_tuples.next().unwrap(),
                assigment_tuples.next().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let assignments = parse_input(input);
    assignments
        .iter()
        .filter(|(a, b)| a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1)
        .count()
}

pub fn part_two(input: &str) -> usize {
    let assignments = parse_input(input);
    assignments
        .iter()
        .filter(|(a, b)| !(a.1 < b.0 || a.0 > b.1))
        .count()
}
