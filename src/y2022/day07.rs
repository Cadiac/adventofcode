use std::collections::HashMap;

use serde_scan::scan;

use crate::solution::{AocError, Solution};

pub struct Day07;

#[derive(Debug, Clone, Default)]
struct FsNode {
    directories: HashMap<String, FsNode>,
    files: HashMap<String, usize>,
}

fn parse(input: &str) -> Result<FsNode, AocError> {
    let mut working_dir: Vec<String> = Vec::new();
    let mut root = FsNode::default();

    for line in input.lines() {
        if line.starts_with('$') {
            if line.starts_with("$ ls") {
                continue;
            }

            let path = scan!("$ cd {}" <- line).map_err(|err| AocError::parse(line, err))?;

            if path == ".." {
                working_dir.pop();
            } else {
                working_dir.push(path);
            }
        } else {
            let mut target = &mut root;
            for path in working_dir.iter() {
                target = target.directories.entry(path.clone()).or_default();
            }

            if line.starts_with("dir") {
                let name = scan!("dir {}" <- line).map_err(|err| AocError::parse(line, err))?;
                target
                    .directories
                    .entry(name)
                    .or_insert_with(FsNode::default);
            } else {
                let (size, name) =
                    scan!("{} {}" <- line).map_err(|err| AocError::parse(line, err))?;
                target.files.insert(name, size);
            }
        }
    }

    Ok(root)
}

fn find_directory_sizes(node: &FsNode) -> Vec<usize> {
    let mut total_size = 0;

    // Begin by determining the sizes of sub directories and their children
    let mut sizes: Vec<usize> = node
        .directories
        .values()
        .flat_map(|sub_dir| {
            let children = find_directory_sizes(sub_dir);
            // The last element contains the size of the whole sub directory,
            // the elements before it are its children
            if let Some(last) = children.last() {
                total_size += last;
            }
            children
        })
        .collect();

    // Add the sizes of files contained within this directory
    total_size += node.files.values().sum::<usize>();

    sizes.push(total_size);

    sizes
}

impl Solution for Day07 {
    type F = usize;
    type S = usize;

    fn meta(&self) -> (u32, u32) {
        (7, 2022)
    }

    fn default_input(&self) -> &'static str {
        include_str!("../../inputs/2022/day07.txt")
    }

    fn part_1(&self, input: &str) -> Result<usize, AocError> {
        let fs = parse(input)?;

        Ok(find_directory_sizes(&fs)
            .into_iter()
            .filter(|size| *size <= 100000)
            .sum())
    }

    fn part_2(&self, input: &str) -> Result<usize, AocError> {
        let fs = parse(input)?;
        let mut sizes = find_directory_sizes(&fs);

        let total_available = 70000000;
        let update_size = 30000000;

        let used_space = sizes.last().ok_or_else(|| AocError::logic("empty fs"))?;
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
