advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    // I'm running on Windows, so let's eliminate carriage returns
    let input = input.replace('\r', "");
    let grids = input.split("\n\n");
    let answer: usize = grids
        .into_iter()
        .map(|grid| {
            // there are only two kinds of value in the grid: ash and mirrors. This means I can convert each cell to a boolean or even a bit in a composite number (where every cell is a distinct bit)
            // I have developed an impression over time that there is a noteworthy speed and storage benefit to doing bit magic whenever possible (depending on how big the input data is, at least). Let's give it a shot here and use composite numbers instead of vecs of booleans
            // let's convert the grid into a list of rows and a list of columns. It'll still be O(n) for both, and it's technically duplicating information, but it seems reasonable to do here since I'm mapping each rows and columns to a number anyway, and it will make equality comparisons really easy
            let row_count = grid.lines().count();
            let col_count = grid.lines().take(1).next().unwrap().len();

            let (empty_rows, empty_cols): (Vec<usize>, Vec<usize>) =
                (vec![0; row_count], vec![0; col_count]);

            let (rows, cols) = grid.lines().enumerate().fold(
                (empty_rows, empty_cols),
                |(mut rows, mut cols), (row_index, line)| {
                    line.chars().enumerate().for_each(|(col_index, c)| {
                        // we're relying on the assumption that there's only '.' and '#' in the grid
                        let bit = (c == '.') as usize;
                        rows[row_index] += bit * 2_usize.pow(col_index as u32);
                        cols[col_index] += bit * 2_usize.pow(row_index as u32);
                    });

                    (rows, cols)
                },
            );
            // println!("\n{}\n", grid);
            // rows.iter().for_each(|row| println!("{:09b}", row));
            // println!();
            // cols.iter().for_each(|col| println!("{:07b}", col));

            if let Some(i) = find_reflecting_line_index(&rows) {
                return 100 * i;
            }

            if let Some(i) = find_reflecting_line_index(&cols) {
                return i;
            }

            0
        })
        .sum();

    Some(answer)
}

/*
 * lines can either be a list of rows or a list of columns (wherein each row or column has already been mapped to a usize)
 * the strategy is to
 * 1. find the first index for a row or column such that the next row or column matches
 * 2. determine the "necessary reflection radius". It's either the index plus 1 or the total number of rows/columns minus the index minus 1  . For example, if you find a candidate for reflection on row index 2 and there are 7 rows, you know there's only space for 3 rows to be reflected both above and below the prospective reflecting line. For another example, if you find a candidate for reflection on row index 2 but there are only 4 rows, you know that there's only space for 1 row to be reflected both above and below the prospective reflecting line
 * 3. for each step of the radius, crawl outward from the index and its matching index + 1, confirming every successive pair of indexes have matching lines as well
 */
fn find_reflecting_line_index(lines: &[usize]) -> Option<usize> {
    let line_count = lines.len();
    for i in 0..line_count - 1 {
        if lines[i] == lines[i + 1] {
            let necessary_reflection_radius = (i + 1).min(line_count - i - 1);
            if (1..necessary_reflection_radius)
                .all(|radius| lines[i - radius] == lines[i + 1 + radius])
            {
                return Some(i + 1);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    // I'm running on Windows, so let's eliminate carriage returns
    let input = input.replace('\r', "");
    let grids = input.split("\n\n");
    // println!("grid count: {}", grids.clone().count());
    // println!("grids: {:?}", grids.clone());
    let answer: usize = grids
        .into_iter()
        .map(|grid| {
            // in classic fashion, part two deviates from part one in a way that would require me to retroactively change my part one code if I want to reuse it here. But I hate doing that because it often makes understanding the part one code needlessly difficult, and it certainly would here, so screw that
            // I'm going to store the rows and columns as numbers again, the same way
            // I'll just make a variant of find_reflecting_line_index that requires the smudge

            let row_count = grid.lines().count();
            let col_count = grid.lines().take(1).next().unwrap().len();

            let (empty_rows, empty_cols): (Vec<usize>, Vec<usize>) =
                (vec![0; row_count], vec![0; col_count]);

            let (rows, cols) = grid.lines().enumerate().fold(
                (empty_rows, empty_cols),
                |(mut rows, mut cols), (row_index, line)| {
                    line.chars().enumerate().for_each(|(col_index, c)| {
                        let bit = (c == '.') as usize;
                        rows[row_index] += bit * 2_usize.pow(col_index as u32);
                        cols[col_index] += bit * 2_usize.pow(row_index as u32);
                    });

                    (rows, cols)
                },
            );

            if let Some(i) = find_reflecting_line_index_with_a_smudge(&rows) {
                return 100 * i;
            }

            if let Some(i) = find_reflecting_line_index_with_a_smudge(&cols) {
                return i;
            }

            0
        })
        .sum();

    Some(answer)
}

// this is using the same strategy as find_reflecting_line_index except I need to break down the bitwise diff and ensure there's only one bit of difference overall
fn find_reflecting_line_index_with_a_smudge(lines: &[usize]) -> Option<usize> {
    let line_count = lines.len();
    for i in 0..line_count - 1 {
        let necessary_reflection_radius = (i + 1).min(line_count - i - 1);
        // thank goodness for the usize type's count_ones function
        if 1_u32
            == (0..necessary_reflection_radius)
                .map(|radius| {
                    let bitwise_diff = lines[i - radius] ^ lines[i + 1 + radius];
                    bitwise_diff.count_ones()
                })
                .sum()
        {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
