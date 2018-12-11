const INPUT_FILE: &str = include_str!("../input.txt");

pub struct Node {
  children: Vec<Node>,
  metadata: Vec<i32>,
  length: usize,
}

fn create_node(slice: &[i32]) -> Node {
    let index = 0;
    if index + 1 > slice.len() {
        panic!("Parsing error at index {}", index);
    };

    let mut current_node = Node{
        children: Vec::new(),
        metadata: Vec::new(),
        length: 2,
    };

    let children_count = slice[0];
    let metadata_count = slice[1];

    // Add the metadata values count to total length of this node
    current_node.length += metadata_count as usize;

    let mut unprocessed_slice = &slice[2..slice.len()];

    for _child in 0..children_count {
        let child_node = create_node(unprocessed_slice);
        current_node.length += child_node.length;
        unprocessed_slice = &unprocessed_slice[child_node.length..];
        current_node.children.push(child_node);
    }

    if unprocessed_slice.len() < metadata_count as usize {
        panic!("Invalid metadata length, {} was less than {}", unprocessed_slice.len(), metadata_count);
    }

    let mut unprocessed_iter = unprocessed_slice.iter();

    for _metadata in 0..metadata_count {
        let value = unprocessed_iter.next().expect("Should exist");
        current_node.metadata.push(value.clone());
    }

    current_node
}

fn metadata_sum(node: &Node) -> i32 {
    node.metadata.iter().sum::<i32>() + node.children.iter().map(|c| metadata_sum(c)).sum::<i32>()
}

fn part_1(file: &str) -> i32 {
    let data: Vec<i32> = file
        .split(" ")
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    let root = create_node(&data[..]);
    metadata_sum(&root)
}

fn main() {
    let part1_result = part_1(INPUT_FILE);
    
    println!("Part 1: {}", part1_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../test/example.txt");

    #[test]
    fn it_solves_day07_part1_example() {
        assert_eq!(part_1(TEST_FILE), 138);
    }
}
