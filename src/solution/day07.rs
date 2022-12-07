use std::collections::HashMap;

use crate::solution::{AocError, Solution};

pub struct Day07;

#[derive(Debug, Clone, Default)]
struct FsNode {
    directories: HashMap<String, FsNode>,
    files: HashMap<String, usize>,
}

fn parse(input: &str) -> FsNode {
    let mut working_dir: Vec<String> = Vec::new();
    let mut root = FsNode::default();

    let mut lines_iter = input.lines();
    let mut current = lines_iter.next();

    while let Some(line) = current {
        if line.starts_with("$ ls") {
            let mut target = &mut root;
            for path in working_dir.iter() {
                target = target.directories.entry(path.clone()).or_default();
            }

            loop {
                current = lines_iter.next();

                match current {
                    Some(output) => {
                        // End of the output, next command begins
                        if output.starts_with("$") {
                            break;
                        }

                        if output.starts_with("dir ") {
                            if let Ok(name) = serde_scan::scan!("dir {}" <- output) {
                                if !target.directories.contains_key(&name) {
                                    target.directories.insert(name, FsNode::default());
                                }
                                continue;
                            }
                        } else if let Ok((size, name)) = serde_scan::scan!("{} {}" <- output) {
                            target.files.insert(name, size);
                            continue;
                        }

                        unreachable!()
                    }
                    None => break,
                }
            }
        } else if line.starts_with("$ cd ..") {
            working_dir.pop();
            current = lines_iter.next();
        } else if let Ok(path) = serde_scan::scan!("$ cd {}" <- line) {
            working_dir.push(path);
            current = lines_iter.next();
        } else {
            unreachable!()
        }
    }

    root
}

fn find_directory_sizes(node: &FsNode) -> Vec<usize> {
    let mut total_size = 0;

    // Begin by determining the sizes of sub directories and their children
    let mut sizes: Vec<usize> = node
        .directories
        .values()
        .map(|sub_dir| {
            let children = find_directory_sizes(sub_dir);
            // The last element contains the size of the sub directory,
            // the elements before it are its children
            if let Some(last) = children.last() {
                total_size += last;
            }
            children
        })
        .flatten()
        .collect();

    // Add the sizes of files contained within this directory
    total_size += node.files.values().sum::<usize>();

    sizes.push(total_size);

    sizes
}

impl Solution for Day07 {
    type F = usize;
    type S = usize;

    fn name(&self) -> &'static str {
        "Day 07"
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        Ok(find_directory_sizes(&parse(input))
            .into_iter()
            .filter(|size| *size <= 100000)
            .sum())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let mut sizes = find_directory_sizes(&parse(input));

        let total_available = 70000000;
        let update_size = 30000000;

        let used_space = sizes.last().unwrap();
        let required_space = update_size - (total_available - used_space);

        sizes.sort();

        sizes
            .into_iter()
            .find(|size| *size >= required_space)
            .ok_or_else(|| AocError::logic("no suitable directory to delete"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        assert_eq!(
            Day07.part_1(
                "$ cd /\n\
                $ ls\n\
                dir a\n\
                14848514 b.txt\n\
                8504156 c.dat\n\
                dir d\n\
                $ cd a\n\
                $ ls\n\
                dir e\n\
                29116 f\n\
                2557 g\n\
                62596 h.lst\n\
                $ cd e\n\
                $ ls\n\
                584 i\n\
                $ cd ..\n\
                $ cd ..\n\
                $ cd d\n\
                $ ls\n\
                4060174 j\n\
                8033020 d.log\n\
                5626152 d.ext\n\
                7214296 k"
            ),
            Ok(95437)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day07.part_2(
                "$ cd /\n\
                $ ls\n\
                dir a\n\
                14848514 b.txt\n\
                8504156 c.dat\n\
                dir d\n\
                $ cd a\n\
                $ ls\n\
                dir e\n\
                29116 f\n\
                2557 g\n\
                62596 h.lst\n\
                $ cd e\n\
                $ ls\n\
                584 i\n\
                $ cd ..\n\
                $ cd ..\n\
                $ cd d\n\
                $ ls\n\
                4060174 j\n\
                8033020 d.log\n\
                5626152 d.ext\n\
                7214296 k"
            ),
            Ok(24933642)
        );
    }
}
