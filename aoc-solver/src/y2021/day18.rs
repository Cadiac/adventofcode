use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use crate::solution::{AocError, Solution};

pub struct Day18;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Pair,
    Value(u32),
}

#[derive(Debug, Clone)]
struct BTNode {
    parent: Option<Weak<RefCell<BTNode>>>,
    left: Option<Rc<RefCell<BTNode>>>,
    right: Option<Rc<RefCell<BTNode>>>,
    op: Op,
}

impl BTNode {
    fn new_empty_pair() -> Rc<RefCell<BTNode>> {
        Rc::new(RefCell::new(BTNode {
            parent: None,
            left: None,
            right: None,
            op: Op::Pair,
        }))
    }
    fn new_pair(
        left: Option<Rc<RefCell<BTNode>>>,
        right: Option<Rc<RefCell<BTNode>>>,
    ) -> Rc<RefCell<BTNode>> {
        Rc::new(RefCell::new(BTNode {
            parent: None,
            left,
            right,
            op: Op::Pair,
        }))
    }
    fn new_value(value: u32) -> Rc<RefCell<BTNode>> {
        Rc::new(RefCell::new(BTNode {
            parent: None,
            left: None,
            right: None,
            op: Op::Value(value),
        }))
    }
    fn new_value_with_parent(value: u32, parent: &Rc<RefCell<BTNode>>) -> Rc<RefCell<BTNode>> {
        Rc::new(RefCell::new(BTNode {
            parent: Some(Rc::downgrade(parent)),
            left: None,
            right: None,
            op: Op::Value(value),
        }))
    }
}

impl PartialEq for BTNode {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.op == other.op
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BinaryTree {
    head: Option<Rc<RefCell<BTNode>>>,
}

impl std::fmt::Display for BinaryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BinaryTree::unparse(self.head.as_ref().unwrap()))
    }
}

impl BinaryTree {
    fn parse(input: &str) -> BinaryTree {
        let mut depth = 0;
        let mut stack: Vec<(usize, Rc<RefCell<BTNode>>)> = Vec::new();
        for c in input.chars() {
            if c == '[' {
                let node = BTNode::new_empty_pair();
                if let Some((_depth, parent)) = stack.last() {
                    let mut node_mut = node.borrow_mut();
                    node_mut.parent = Some(Rc::downgrade(parent));
                }
                stack.push((depth, node));
                depth += 1;
            } else if c == ']' {
                // Empty pair "[]" is illegal
                assert!(stack.len() >= 2);
                let (r_depth, right) = stack.pop().unwrap();
                let (l_depth, left) = stack.pop().unwrap();

                if left.borrow().op == Op::Pair && l_depth != r_depth {
                    // There was no left node
                    let parent = left;
                    right.borrow_mut().parent = Some(Rc::downgrade(&parent));
                    parent.borrow_mut().right = Some(right);

                    stack.push((depth, parent));
                } else {
                    let (depth, parent) = stack.pop().unwrap();
                    right.borrow_mut().parent = Some(Rc::downgrade(&parent));
                    left.borrow_mut().parent = Some(Rc::downgrade(&parent));
                    parent.borrow_mut().right = Some(right);
                    parent.borrow_mut().left = Some(left);

                    stack.push((depth, parent));
                };

                depth -= 1;
            } else if c.is_digit(10) {
                let value = c.to_digit(10).unwrap();
                let node = BTNode::new_value(value);
                stack.push((depth, node));
            }
        }
        // Now the stack should only contain the head
        assert_eq!(stack.len(), 1);

        BinaryTree {
            head: Some(stack.pop().unwrap().1),
        }
    }

    // For debugging purposes
    fn unparse(node: &Rc<RefCell<BTNode>>) -> String {
        if let Op::Value(value) = node.borrow().op {
            return value.to_string();
        }
        let mut output = String::new();
        if let Some(left) = &node.borrow().left {
            output += "[";
            output += BinaryTree::unparse(left).as_str();
        }
        if let Some(right) = &node.borrow().right {
            output += ",";
            output += BinaryTree::unparse(right).as_str();
            output += "]";
        }
        output
    }

    fn add(&mut self, other: BinaryTree) {
        let head = BTNode::new_pair(std::mem::take(&mut self.head), other.head);
        head.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&head));
        head.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&head));

        self.head = Some(head);
    }

    fn find_leftmost_explosive(&self) -> Option<Rc<RefCell<BTNode>>> {
        BinaryTree::find_explosive_recursive(self.head.as_ref().unwrap(), 0)
    }

    fn find_explosive_recursive(
        node: &Rc<RefCell<BTNode>>,
        depth: usize,
    ) -> Option<Rc<RefCell<BTNode>>> {
        if depth == 4 && node.borrow().op == Op::Pair {
            return Some(node.clone());
        }

        if let Some(left) = &node.borrow().left {
            let found = BinaryTree::find_explosive_recursive(left, depth + 1);
            if found.is_some() {
                return found;
            }
        }
        if let Some(right) = &node.borrow().right {
            return BinaryTree::find_explosive_recursive(right, depth + 1);
        }

        None
    }

    fn find_leftmost_splittable(&self) -> Option<Rc<RefCell<BTNode>>> {
        BinaryTree::find_splittable_recursive(self.head.as_ref().unwrap())
    }

    fn find_splittable_recursive(node: &Rc<RefCell<BTNode>>) -> Option<Rc<RefCell<BTNode>>> {
        if let Op::Value(value) = node.borrow().op {
            if value >= 10 {
                return Some(node.clone());
            }
        }

        if let Some(left) = &node.borrow().left {
            let found = BinaryTree::find_splittable_recursive(left);
            if found.is_some() {
                return found;
            }
        }
        if let Some(right) = &node.borrow().right {
            return BinaryTree::find_splittable_recursive(right);
        }

        None
    }

    fn add_to_ancestors_left(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(parent_weak) = &node.borrow().parent {
            if let Some(parent) = parent_weak.upgrade() {
                if let Some(left) = &parent.borrow().left {
                    if Rc::ptr_eq(left, &node) {
                        BinaryTree::add_to_ancestors_left(parent.clone(), value);
                    } else {
                        let op = left.borrow().op;
                        match op {
                            Op::Pair => {
                                BinaryTree::add_to_offspring_right(left.clone(), value);
                            }
                            Op::Value(current_value) => {
                                left.borrow_mut().op = Op::Value(current_value + value);
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_to_ancestors_right(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(parent_weak) = &node.borrow().parent {
            if let Some(parent) = parent_weak.upgrade() {
                if let Some(right) = &parent.borrow().right {
                    if Rc::ptr_eq(right, &node) {
                        BinaryTree::add_to_ancestors_right(parent.clone(), value);
                    } else {
                        let op = right.borrow().op;
                        match op {
                            Op::Pair => {
                                BinaryTree::add_to_offspring_left(right.clone(), value);
                            }
                            Op::Value(current_value) => {
                                right.borrow_mut().op = Op::Value(current_value + value);
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_to_offspring_left(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(left) = &node.borrow().left {
            let op = left.borrow().op;
            match op {
                Op::Pair => {
                    BinaryTree::add_to_offspring_left(left.clone(), value);
                }
                Op::Value(current_value) => {
                    left.borrow_mut().op = Op::Value(current_value + value);
                }
            }
        }
    }

    fn add_to_offspring_right(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(right) = &node.borrow().right {
            let op = right.borrow().op;
            match op {
                Op::Pair => {
                    BinaryTree::add_to_offspring_right(right.clone(), value);
                }
                Op::Value(current_value) => {
                    right.borrow_mut().op = Op::Value(current_value + value);
                }
            }
        }
    }

    fn reduce(&self) {
        loop {
            if let Some(node) = self.find_leftmost_explosive() {
                BinaryTree::explode(&node);
            } else if let Some(node) = self.find_leftmost_splittable() {
                BinaryTree::split(&node);
            } else {
                break;
            }
        }
    }

    fn magnitude(&self) -> u32 {
        BinaryTree::magnitude_recursive(self.head.as_ref().unwrap())
    }

    fn magnitude_recursive(node: &Rc<RefCell<BTNode>>) -> u32 {
        let op = node.borrow().op;
        let mut magnitude = 0;
        match op {
            Op::Value(value) => value,
            Op::Pair => {
                if let Some(left) = node.borrow().left.clone() {
                    magnitude += 3 * BinaryTree::magnitude_recursive(&left);
                }

                if let Some(right) = node.borrow().right.clone() {
                    magnitude += 2 * BinaryTree::magnitude_recursive(&right);
                }

                magnitude
            }
        }
    }

    fn explode(node: &Rc<RefCell<BTNode>>) {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        if let Some(left) = left {
            if let Op::Value(value) = left.borrow().op {
                BinaryTree::add_to_ancestors_left(left.clone(), value);
            }
        }

        if let Some(right) = right {
            if let Op::Value(value) = right.borrow().op {
                BinaryTree::add_to_ancestors_right(right.clone(), value);
            }
        }

        let mut node_mut = node.borrow_mut();
        node_mut.op = Op::Value(0);
        node_mut.left = None;
        node_mut.right = None;
    }

    fn split(node: &Rc<RefCell<BTNode>>) {
        let op = node.borrow().op;
        if let Op::Value(value) = op {
            let left = BTNode::new_value_with_parent(value / 2, node);
            let right = BTNode::new_value_with_parent((value + 1) / 2, node);

            let mut node_mut = node.borrow_mut();
            node_mut.op = Op::Pair;
            node_mut.left = Some(left);
            node_mut.right = Some(right);
        } else {
            panic!("Cannot split a non-value node");
        }
    }
}

fn add_list(input: &str) -> BinaryTree {
    let mut lines = input.lines().map(BinaryTree::parse);
    let mut result = lines.next().unwrap();

    for number in lines {
        result.add(number);
        result.reduce();
    }

    result
}

impl Solution for Day18 {
    type F = u32;
    type S = u32;

    fn default_input(&self) -> &'static str {
        include_str!("../../../inputs/2021/day18.txt")
    }

    fn part_1(&self, input: &str) -> Result<Self::F, AocError> {
        let result = add_list(input);
        let magnitude = result.magnitude();

        Ok(magnitude)
    }

    fn part_2(&self, input: &str) -> Result<Self::S, AocError> {
        let mut largest_magnitude = 0;

        for (first, second) in input.lines().tuple_combinations() {
            let mut binary_tree = BinaryTree::parse(first);
            binary_tree.add(BinaryTree::parse(second));
            binary_tree.reduce();
            let magnitude = binary_tree.magnitude();
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }

            let mut binary_tree = BinaryTree::parse(second);
            binary_tree.add(BinaryTree::parse(first));
            binary_tree.reduce();
            let magnitude = binary_tree.magnitude();
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }
        }

        Ok(largest_magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_snailfish_numbers_1() {
        assert_eq!(
            BinaryTree::parse("[1,2]"),
            BinaryTree {
                head: Some(BTNode::new_pair(
                    Some(BTNode::new_value(1)),
                    Some(BTNode::new_value(2)),
                ))
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_2() {
        assert_eq!(
            BinaryTree::parse("[[1,2],3]"),
            BinaryTree {
                head: Some(BTNode::new_pair(
                    Some(BTNode::new_pair(
                        Some(BTNode::new_value(1)),
                        Some(BTNode::new_value(2)),
                    )),
                    Some(BTNode::new_value(3))
                ))
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_3() {
        assert_eq!(
            BinaryTree::parse("[9,[8,7]]"),
            BinaryTree {
                head: Some(BTNode::new_pair(
                    Some(BTNode::new_value(9)),
                    Some(BTNode::new_pair(
                        Some(BTNode::new_value(8)),
                        Some(BTNode::new_value(7)),
                    )),
                ))
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_4() {
        assert_eq!(
            BinaryTree::parse("[[1,9],[8,5]]"),
            BinaryTree {
                head: Some(BTNode::new_pair(
                    Some(BTNode::new_pair(
                        Some(BTNode::new_value(1)),
                        Some(BTNode::new_value(9)),
                    )),
                    Some(BTNode::new_pair(
                        Some(BTNode::new_value(8)),
                        Some(BTNode::new_value(5)),
                    )),
                ))
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_5() {
        assert_eq!(
            BinaryTree::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"),
            BinaryTree {
                head: Some(BTNode::new_pair(
                    Some(BTNode::new_pair(
                        Some(BTNode::new_pair(
                            Some(BTNode::new_pair(
                                Some(BTNode::new_value(1)),
                                Some(BTNode::new_value(2)),
                            )),
                            Some(BTNode::new_pair(
                                Some(BTNode::new_value(3)),
                                Some(BTNode::new_value(4)),
                            )),
                        )),
                        Some(BTNode::new_pair(
                            Some(BTNode::new_pair(
                                Some(BTNode::new_value(5)),
                                Some(BTNode::new_value(6)),
                            )),
                            Some(BTNode::new_pair(
                                Some(BTNode::new_value(7)),
                                Some(BTNode::new_value(8)),
                            )),
                        )),
                    )),
                    Some(BTNode::new_value(9)),
                ))
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_6() {
        BinaryTree::parse("[1,2]");
        BinaryTree::parse("[[1,2],3]");
        BinaryTree::parse("[9,[8,7]]");
        BinaryTree::parse("[[1,9],[8,5]]");
        BinaryTree::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        BinaryTree::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        BinaryTree::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn it_adds_two_snailfish_numbers() {
        let mut binary_tree = BinaryTree::parse("[1,2]");
        binary_tree.add(BinaryTree::parse("[[3,4],5]"));
        assert_eq!(binary_tree, BinaryTree::parse("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn it_finds_explosive_pairs_1() {
        let binary_tree = BinaryTree::parse("[[[[[9,8],1],2],3],4]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        assert!(explosive_pair.is_some());
        assert_eq!(explosive_pair, BinaryTree::parse("[9,8]").head);
    }

    #[test]
    fn it_finds_explosive_pairs_2() {
        let binary_tree = BinaryTree::parse("[[[[1,[9,8]],2],3],4]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        assert_eq!(explosive_pair, BinaryTree::parse("[9,8]").head);
    }

    #[test]
    fn it_finds_explosive_pairs_3() {
        let binary_tree = BinaryTree::parse("[7,[6,[5,[4,[3,2]]]]]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        assert_eq!(explosive_pair, BinaryTree::parse("[3,2]").head);
    }

    #[test]
    fn it_finds_explosive_pairs_4() {
        let binary_tree = BinaryTree::parse("[[[[1,9],2],3],4]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        assert_eq!(explosive_pair, None);
    }

    #[test]
    fn it_explodes_examples_1() {
        let binary_tree = BinaryTree::parse("[[[[[9,8],1],2],3],4]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, BinaryTree::parse("[[[[0,9],2],3],4]"));
    }

    #[test]
    fn it_explodes_examples_2() {
        let binary_tree = BinaryTree::parse("[7,[6,[5,[4,[3,2]]]]]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, BinaryTree::parse("[7,[6,[5,[7,0]]]]"));
    }

    #[test]
    fn it_explodes_examples_3() {
        let binary_tree = BinaryTree::parse("[[6,[5,[4,[3,2]]]],1]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, BinaryTree::parse("[[6,[5,[7,0]]],3]"));
    }

    #[test]
    fn it_explodes_examples_4() {
        let binary_tree = BinaryTree::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(
            binary_tree,
            BinaryTree::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
    }

    #[test]
    fn it_explodes_examples_5() {
        let binary_tree = BinaryTree::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(
            binary_tree,
            BinaryTree::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn it_splits_example_manually() {
        let mut binary_tree = BinaryTree::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        binary_tree.add(BinaryTree::parse("[1,1]"));

        assert_eq!(
            binary_tree,
            BinaryTree::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        );

        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());

        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());

        let splittable = binary_tree.find_leftmost_splittable();
        BinaryTree::split(&splittable.unwrap());

        let splittable = binary_tree.find_leftmost_splittable();
        BinaryTree::split(&splittable.unwrap());
        assert_eq!(
            binary_tree,
            BinaryTree::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
        );

        let explosive_pair = binary_tree.find_leftmost_explosive();
        BinaryTree::explode(&explosive_pair.unwrap());

        assert_eq!(
            binary_tree,
            BinaryTree::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn it_adds_list_of_snailfish_numbers_1() {
        assert_eq!(
            add_list("[1,1]\n[2,2]\n[3,3]\n[4,4]"),
            BinaryTree::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );
    }

    #[test]
    fn it_adds_list_of_snailfish_numbers_2() {
        assert_eq!(
            add_list("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]"),
            BinaryTree::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")
        );
    }

    #[test]
    fn it_adds_list_of_snailfish_numbers_3() {
        assert_eq!(
            add_list("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]"),
            BinaryTree::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        );
    }

    #[test]
    fn it_adds_list_of_snailfish_numbers_4() {
        let final_sum = add_list(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
             [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
             [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
             [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
             [7,[5,[[3,8],[1,4]]]]\n\
             [[2,[2,2]],[8,[8,1]]]\n\
             [2,9]\n\
             [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
             [[[5,[7,4]],7],1]\n\
             [[[[4,2],2],6],[8,7]]",
        );

        assert_eq!(
            final_sum,
            BinaryTree::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    #[rustfmt::skip]
    fn it_solves_magnitude_examples() {
        assert_eq!(BinaryTree::parse("[9,1]").magnitude(), 29);
        assert_eq!(BinaryTree::parse("[1,9]").magnitude(), 21);
        assert_eq!(BinaryTree::parse("[[9,1],[1,9]]").magnitude(), 129);
        assert_eq!(BinaryTree::parse("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(BinaryTree::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(BinaryTree::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(BinaryTree::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(BinaryTree::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(BinaryTree::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
        assert_eq!(BinaryTree::parse("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]").magnitude(), 3993);
    }

    #[test]
    fn it_solves_part1_full_example() {
        assert_eq!(
            Day18.part_1(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
                 [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
                 [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
                 [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
                 [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
                 [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
                 [[[[5,4],[7,7]],8],[[8,3],8]]\n\
                 [[9,3],[[9,9],[6,[4,9]]]]\n\
                 [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
                 [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            Ok(4140)
        );
    }

    #[test]
    fn it_solves_part2_example() {
        assert_eq!(
            Day18.part_2(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
                 [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
                 [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
                 [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
                 [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
                 [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
                 [[[[5,4],[7,7]],8],[[8,3],8]]\n\
                 [[9,3],[[9,9],[6,[4,9]]]]\n\
                 [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
                 [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            Ok(3993)
        );
    }
}
