use std::vec;

#[derive(Debug)]
struct Directory {
    size: usize,
    children: Vec<Directory>,
}

fn parse_input(input: &str) -> Directory {
    // let's just treat this thing as a tree
    // we'll assume that "cd /" happens once at the start and can be ignored
    // we'll also assume that the pattern is always a single "ls" followed by one "cd (.)" for each directory
    // when we see "ls", we'll just sum up the file sizes for the directory and then loop
    // when we see "cd ..", we'll return from the recursive function
    // when we see "cd (.)", we'll add a new child with the output of the recursive function

    let mut lines = input.lines().map(|s| s.to_owned());
    // eat the "cd /"
    lines.next();
    let mut root_directory = recursively_parse(&mut lines);
    // println!("{:?}", root_directory);

    // so now I want to traverse and mutate the tree to add the sum of its children's sizes to its own
    recursively_update_and_get_size(&mut root_directory);
    // println!("{:?}", root_directory);
    root_directory
}

fn recursively_parse(lines: &mut impl Iterator<Item = String>) -> Directory {
    // the first line is going to be ls
    // we can hydrate the directory size
    // we know how many children there will be if we count them too
    // so we can perfectly iterate once for every child, then read a "cd .." and return

    // eat the "ls"
    lines.next();

    let mut num_children = 0;
    let mut children = vec![];
    let mut size = 0;

    // scan over the ls output, summing size and counting children as we go
    // when the ls output is done, either lines.next() will be None, or the line will be some `$ cd ?`
    // we'll use first_line_after_ls to track that. It will remain None if the final line of lines was just part of ls output, and if we see a `$ cd ?` then we'll put that into first_line_after_ls and figure out whether to forget it if it's "cd .." or iterate over children if it's "cd (.)"
    while let Some(line) = lines.next() {
        // println!("line after ls: {}", line);
        // if we are looking at a command line that isn't ls output, stop looping and figure out how to deal with that
        if line.starts_with("$") {
            break;
        }
        // otherwise we are looking at ls output
        let mut line_words = line.split(" ");
        let first = line_words.next().unwrap();
        // println!("first word in ls line: {}, {}", first, "dir" == first);
        if let Ok(file_size) = first.parse::<usize>() {
            // if the ls output line is a file, let's add that file size to our sum
            size += file_size;
            // println!(
            //     "current directory size is now {}",
            //     current_directory.size
            // );
        } else if "dir" == first {
            // if the ls outut line is a directory, let's increment the num_children
            num_children += 1;
        } else {
            panic!("a ls output line started with something other than a number or 'dir'")
        }
    }

    // the current line (the one we have consumed with lines.next()) following the ls output is either "cd (.)" for a child or "cd .."
    // if we have children, it must be the former
    if num_children > 0 {
        // so if num_children > 0, assume that this current line is "cd (.)" to process a child
        // iterate over children
        for _i in 0..num_children {
            children.push(recursively_parse(lines));
            // the next line is either "cd (.)" for the next child or "cd .."
            lines.next();
        }
    }

    // at this point, the current line must be "cd .."

    Directory { size, children }
}

fn recursively_update_and_get_size(directory: &mut Directory) -> usize {
    directory.size += directory
        .children
        .iter_mut()
        .map(|c| recursively_update_and_get_size(c))
        .sum::<usize>();
    directory.size
}

fn recursively_get_sizes_within_threshold(directory: &Directory, threshold: usize) -> Vec<usize> {
    let mut v = vec![];
    if directory.size <= threshold {
        v.push(directory.size);
    }
    directory.children.iter().for_each(|c| {
        let mut children_sizes_below_threshold =
            recursively_get_sizes_within_threshold(c, threshold);
        v.append(&mut children_sizes_below_threshold);
    });
    v
}

pub fn part_one(input: &str) -> usize {
    let root_directory = parse_input(input);

    let sizes_below_threshold = recursively_get_sizes_within_threshold(&root_directory, 100000);
    // println!("{:?}", sizes_below_threshold);

    sizes_below_threshold.iter().sum()
}

fn recursively_get_sizes(directory: &Directory) -> Vec<usize> {
    let mut v = vec![directory.size];
    directory
        .children
        .iter()
        .for_each(|c| v.append(&mut recursively_get_sizes(c)));
    v
}
pub fn part_two(input: &str) -> usize {
    let root_directory = parse_input(input);
    let total_space = 70000000;
    let unused_space = total_space - root_directory.size;
    let unused_space_needed = 30000000;
    if unused_space >= unused_space_needed {
        return 0;
    }
    let target_size_to_delete = unused_space_needed - unused_space;

    // we have to find the smallest directory that's greater than or equal to this target size
    // I'll just traverse for a list of directory sizes and then pick the smallest one that meets that criterion

    let sizes = recursively_get_sizes(&root_directory);
    // println!("{:?}", sizes);

    let size_to_delete = sizes
        .iter()
        .filter(|size| **size >= target_size_to_delete)
        .min()
        .unwrap();
    // println!("{}", size_to_delete);

    *size_to_delete
}
