fn parse_input(input: &str) -> (usize, Vec<u32>) {
    (
        input.lines().next().unwrap().len(),
        input.lines().fold(vec![], |mut acc, line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .for_each(|tree_height| acc.push(tree_height));
            acc
        }),
    )
}

fn trees_at_each_index_shorter(height: u32, indexes: Vec<usize>, grid: &Vec<u32>) -> bool {
    indexes.into_iter().all(|index| {
        // println!(
        //     "checking index {} height {}: {}",
        //     index,
        //     grid[index],
        //     grid[index] < height
        // );
        grid[index] < height
    })
}

pub fn part_one(input: &str) -> usize {
    let (row_len, grid) = parse_input(input);
    // let col_len = grid.len() / row_len;

    let number_of_edge_trees = row_len * 2 + (grid.len() / row_len - 2) * 2;

    // println!("{}, {:?}", row_len, grid);

    number_of_edge_trees
        + grid
            .iter()
            .enumerate()
            // filter to inner trees
            .filter(|(i, _)| {
                // first col, last col, first row, last row
                if *i % row_len == 0
                    || *i % row_len == row_len - 1
                    || *i < row_len
                    || *i > grid.len() - row_len
                {
                    return false;
                }
                true
            })
            // filter to trees that are visible
            .filter(|(i, &tree_height)| {
                let row = i / row_len;
                // println!("row: {row}");
                let col = *i % row_len;
                // a tree is visible if tree to any direction is less tall
                // let's concoct the indexes to scan over
                let indexes_above: Vec<usize> =
                    (0..row).map(|index| index * row_len + col).collect();
                // println!("indexes_above: {:?}", indexes_above);
                let indexes_below: Vec<usize> = ((row + 1)..row_len)
                    .map(|index| index * row_len + col)
                    .collect();
                // println!("indexes_below: {:?}", indexes_below);
                let indexes_left: Vec<usize> = ((i - col)..*i).collect();
                // println!("indexes_left: {:?}", indexes_left);
                let indexes_right: Vec<usize> = ((i + 1)..(i + (row_len - col))).collect();
                // println!("indexes_right: {:?}", indexes_right);

                let visible = trees_at_each_index_shorter(tree_height, indexes_above, &grid)
                    || trees_at_each_index_shorter(tree_height, indexes_below, &grid)
                    || trees_at_each_index_shorter(tree_height, indexes_left, &grid)
                    || trees_at_each_index_shorter(tree_height, indexes_right, &grid);

                // println!("{} visible? {}", tree_height, visible);

                visible
            })
            .count()
}

fn get_viewing_distance(height: u32, indexes: Vec<usize>, grid: &Vec<u32>) -> usize {
    let mut count = 0;
    for i in indexes {
        count += 1;
        if grid[i] >= height {
            break;
        }
    }
    count
}

pub fn part_two(input: &str) -> usize {
    let (row_len, grid) = parse_input(input);

    // println!("{}, {:?}", row_len, grid);

    grid.iter()
        .enumerate()
        // might as well filter to inner trees because edge trees have a score of 0
        .filter(|(i, _)| {
            // first col, last col, first row, last row
            if *i % row_len == 0
                || *i % row_len == row_len - 1
                || *i < row_len
                || *i > grid.len() - row_len
            {
                return false;
            }
            true
        })
        // map to scenic score
        .map(|(i, &tree_height)| {
            let row = i / row_len;
            // println!("row: {row}");
            let col = i % row_len;
            // a tree is visible if tree to any direction is less tall
            // let's concoct the indexes to scan over
            let indexes_above: Vec<usize> =
                (0..row).map(|index| index * row_len + col).rev().collect();
            // println!("indexes_above: {:?}", indexes_above);
            let indexes_below: Vec<usize> = ((row + 1)..row_len)
                .map(|index| index * row_len + col)
                .collect();
            // println!("indexes_below: {:?}", indexes_below);
            let indexes_left: Vec<usize> = ((i - col)..i).rev().collect();
            // println!("indexes_left: {:?}", indexes_left);
            let indexes_right: Vec<usize> = ((i + 1)..(i + (row_len - col))).collect();
            // println!("indexes_right: {:?}", indexes_right);

            let scenic_score = get_viewing_distance(tree_height, indexes_above, &grid)
                * get_viewing_distance(tree_height, indexes_below, &grid)
                * get_viewing_distance(tree_height, indexes_left, &grid)
                * get_viewing_distance(tree_height, indexes_right, &grid);

            // println!("{} scenic score? {}", tree_height, scenic_score);

            scenic_score
        })
        .max()
        .unwrap_or(0)
}
