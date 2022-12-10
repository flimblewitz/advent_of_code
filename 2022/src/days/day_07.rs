use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    size: usize,
    children: HashMap<char, Directory>,
}

// let's assume that the input nicely doesn't jump back to the root
// let's assume that `cd ..` means it's done with the directory forever
fn parse_input(input: &str) -> Directory {
    let mut lines = input.lines().map(|s| s.to_owned());
    // skip the cd / line
    lines.next();

    let mut next_line = lines.next();
    let mut root_directory = Directory {
        size: 0,
        children: HashMap::new(),
    };
    while let Some(line) = next_line {
        // println!("main loop is iterating with line {line}");
        next_line = recursively_parse_input(&mut lines, &mut root_directory, Some(line));
    }
    root_directory
}

// returns next line to parse
fn recursively_parse_input(
    lines: &mut impl Iterator<Item = String>,
    current_directory: &mut Directory,
    mut current_line: Option<String>,
) -> Option<String> {
    while let Some(ref line) = current_line {
        // println!("line: {}", line);
        let chars: Vec<_> = line.chars().collect();
        match chars[0] {
            '$' => {
                match chars[2..4] {
                    ['c', 'd'] => {
                        if ['.', '.'] == chars[5..] {
                            // println!("hit '..', leaving");
                            return lines.next();
                        }
                        current_line = lines.next();
                        let child_directory_name = &chars[5];
                        // println!("keys are {:?}", current_directory.children.keys());
                        // println!("key is {}", child_directory_name);
                        // println!(
                        //     "the child directory is {:?}",
                        //     current_directory.children.get(child_directory_name)
                        // );
                        // println!("delving into directory {child_directory_name}");
                        current_line = recursively_parse_input(
                            lines,
                            current_directory
                                .children
                                .get_mut(child_directory_name)
                                .unwrap(),
                            current_line,
                        );
                    }
                    ['l', 's'] => {
                        // iterate over following lines to pump up the current directory
                        let mut current_ls_line = lines.next();
                        while let Some(line) = current_ls_line {
                            // println!("line after ls: {}", line);
                            if line.starts_with("$") {
                                // println!("hit a new $, returning {:?}", current_line);
                                // return current_line;
                                if line.starts_with("$ cd") {
                                    // println!("hit a $, backing out of 'ls'");
                                    current_line = Some(line);
                                    break;
                                }
                                panic!("this $ line is unexpected: {line}");
                            }
                            let mut line_words = line.split(" ");
                            let first = line_words.next().unwrap();
                            // println!("first word in ls line: {}, {}", first, "dir" == first);
                            if let Ok(file_size) = first.parse::<usize>() {
                                current_directory.size += file_size;
                                // println!(
                                //     "current directory size is now {}",
                                //     current_directory.size
                                // );
                            } else if "dir" == first {
                                let next_word_as_char =
                                    line_words.next().unwrap().chars().next().unwrap();
                                current_directory.children.insert(
                                    next_word_as_char,
                                    Directory {
                                        size: 0,
                                        children: HashMap::new(),
                                    },
                                );
                                // println!(
                                //     "just inserted dir {next_word_as_char}, keys are {:?}",
                                //     current_directory.children.keys()
                                // );
                                // println!(
                                //     "the new child is {:?}",
                                //     current_directory.children.get(&next_word_as_char).unwrap()
                                // )
                            } else {
                                panic!("a ls subline started with something other than a number or 'dir'")
                            }
                            current_ls_line = lines.next();
                            // println!("current_ls_line: {current_ls_line:?}");
                            if current_ls_line.is_none() {
                                return None;
                            }
                        }
                    }
                    _ => panic!("a '$' line didn't begin with cd or ls?"),
                }
            }
            _ => panic!(
                "you should be intentionally handling non-command lines after seeing a command"
            ),
        }
    }
    None
}

pub fn part_one(input: &str) -> usize {
    let root_directory = parse_input(input);

    traverse(&root_directory).iter().sum()
}

/*
wrong answers
1898234
1496940
*/
fn traverse(d: &Directory) -> Vec<usize> {
    let mut v: Vec<usize> = d
        .children
        .values()
        .map(|child| traverse(child))
        .flatten()
        .collect();
    // println!("{:?}", v);
    let s: usize = v.iter().sum();
    let total_size_with_children: usize = d.size + s;
    if total_size_with_children < 100000 {
        v.push(total_size_with_children);
    }
    v
}

pub fn part_two(_input: &str) -> String {
    "...".into()
}
