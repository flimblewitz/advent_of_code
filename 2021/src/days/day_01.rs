fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn get_num_contiguous_increases(usize_list: Vec<usize>) -> usize {
    usize_list.windows(2).fold(0, |acc, pair| {
        if pair[0] < pair[1] {
            return acc + 1;
        }
        acc
    })
}

pub fn part_one(input: &str) -> usize {
    get_num_contiguous_increases(parse_input(input))
}

pub fn part_two(input: &str) -> usize {
    let triplet_window_sums: Vec<usize> = parse_input(input)
        .windows(3)
        .map(|triplet| triplet.iter().sum::<usize>())
        .collect();

    get_num_contiguous_increases(triplet_window_sums)
}
