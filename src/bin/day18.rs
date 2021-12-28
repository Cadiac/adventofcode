use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::rc::Weak;

const INPUT_FILE: &str = include_str!("../../inputs/day18.txt");

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

impl BinaryTree {
    fn new(head: Option<Rc<RefCell<BTNode>>>) -> BinaryTree {
        BinaryTree { head }
    }

    fn addition(a: BinaryTree, b: BinaryTree) -> BinaryTree {
        let head = BTNode::new_pair(a.head, b.head);
        head.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&head));
        head.borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .parent = Some(Rc::downgrade(&head));

        BinaryTree::new(Some(head))
    }

    fn find_leftmost_explosive(
        node: &Rc<RefCell<BTNode>>,
        depth: usize,
    ) -> Option<Rc<RefCell<BTNode>>> {
        if depth == 4 && node.borrow().op == Op::Pair {
            return Some(node.clone());
        }

        if let Some(left) = &node.borrow().left {
            let found = BinaryTree::find_leftmost_explosive(left, depth + 1);
            if found.is_some() {
                return found;
            }
        }
        if let Some(right) = &node.borrow().right {
            return BinaryTree::find_leftmost_explosive(right, depth + 1);
        }

        None
    }

    fn find_leftmost_splittable(node: &Rc<RefCell<BTNode>>) -> Option<Rc<RefCell<BTNode>>> {
        if let Op::Value(value) = node.borrow().op {
            if value >= 10 {
                return Some(node.clone());
            }
        }

        if let Some(left) = &node.borrow().left {
            let found = BinaryTree::find_leftmost_splittable(left);
            if found.is_some() {
                return found;
            }
        }
        if let Some(right) = &node.borrow().right {
            return BinaryTree::find_leftmost_splittable(right);
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
                                BinaryTree::add_to_offsprings_right(left.clone(), value);
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

    fn add_to_offsprings_right(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(right) = &node.borrow().right {
            let op = right.borrow().op;
            match op {
                Op::Pair => {
                    BinaryTree::add_to_offsprings_right(right.clone(), value);
                }
                Op::Value(current_value) => {
                    right.borrow_mut().op = Op::Value(current_value + value);
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
                                BinaryTree::add_to_offsprings_left(right.clone(), value);
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

    fn add_to_offsprings_left(node: Rc<RefCell<BTNode>>, value: u32) {
        if let Some(left) = &node.borrow().left {
            let op = left.borrow().op;
            match op {
                Op::Pair => {
                    BinaryTree::add_to_offsprings_left(left.clone(), value);
                }
                Op::Value(current_value) => {
                    left.borrow_mut().op = Op::Value(current_value + value);
                }
            }
        }
    }

    fn reduce(&mut self) {
        unimplemented!();
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

        // Replace the current pair with 0 value nodes
        if let Some(parent) = node.borrow().parent.as_ref().unwrap().upgrade() {
            let mut parent = parent.borrow_mut();

            if Rc::ptr_eq(&parent.left.as_ref().unwrap(), node) {
                parent.left = Some(BTNode::new_value(0));
            } else {
                parent.right = Some(BTNode::new_value(0));
            }
        }
    }

    fn split(node: &Rc<RefCell<BTNode>>) {
        let op = node.borrow().op;
        if let Op::Value(value) = op {
            let left = value / 2;
            let right = (value + 1) / 2;

            let left_node = BTNode::new_value(left);
            left_node.borrow_mut().parent = Some(Rc::downgrade(node));
            let right_node = BTNode::new_value(right);
            right_node.borrow_mut().parent = Some(Rc::downgrade(node));

            let mut node_mut = node.borrow_mut();
            node_mut.op = Op::Pair;
            node_mut.left = Some(left_node);
            node_mut.right = Some(right_node);
        } else {
            panic!("Cannot split a non-value node");
        }
    }

    fn add_to_left(&mut self) {
        unimplemented!();
    }

    fn add_to_right(&mut self) {
        unimplemented!();
    }
}

fn parse(input: &str) -> Result<BinaryTree, String> {
    let mut depth = 0;

    let mut stack: Vec<(usize, Rc<RefCell<BTNode>>)> = Vec::new();

    for c in input.chars() {
        if c == '[' {
            let node = BTNode::new_empty_pair();

            if let Some((_depth, last)) = stack.last() {
                let mut node_mut = node.borrow_mut();
                node_mut.parent = Some(Rc::downgrade(last));
            }

            stack.push((depth, node));
            depth += 1;
        } else if c == ']' {
            // Empty pair "[]" is illegal
            let (right_depth, right) = stack.pop().ok_or("stack pop")?;
            let (left_depth, left) = stack.pop().ok_or("stack pop")?;
            let left_cloned = left.borrow().clone();

            let node_and_depth = match left_cloned {
                BTNode {
                    op: Op::Value(_), ..
                } => {
                    let (depth, node) = stack.pop().ok_or("stack pop")?;

                    {
                        let mut right_mut = right.borrow_mut();
                        right_mut.parent = Some(Rc::downgrade(&node));
                        let mut left_mut = left.borrow_mut();
                        left_mut.parent = Some(Rc::downgrade(&node));
                    }
                    {
                        let mut node_mut = node.borrow_mut();
                        node_mut.right = Some(right);
                        node_mut.left = Some(left);
                    }

                    (depth, node)
                }
                BTNode { op: Op::Pair, .. } => {
                    if left_depth == right_depth {
                        // Same depth, so this was a left node and the
                        // pair node wrapping this is next
                        let (depth, node) = stack.pop().ok_or("stack pop")?;

                        {
                            let mut right_mut = right.borrow_mut();
                            right_mut.parent = Some(Rc::downgrade(&node));
                            let mut left_mut = left.borrow_mut();
                            left_mut.parent = Some(Rc::downgrade(&node));
                        }
                        {
                            let mut node_mut = node.borrow_mut();
                            node_mut.right = Some(right);
                            node_mut.left = Some(left);
                        }

                        (depth, node)
                    } else {
                        // There was no left node
                        let node = left;
                        {
                            let mut right_mut = right.borrow_mut();
                            right_mut.parent = Some(Rc::downgrade(&node));
                        }
                        {
                            let mut node_mut = node.borrow_mut();
                            node_mut.right = Some(right);
                        }
                        (depth, node)
                    }
                }
            };

            // Now push the node back to stack
            stack.push(node_and_depth);
            depth -= 1;
        } else if c.is_digit(10) {
            let value = c.to_digit(10).ok_or("to_digit parsing")?;
            let node = BTNode::new_value(value);

            stack.push((depth, node));
        }
    }

    // Now the stack should only contain the head
    assert_eq!(stack.len(), 1);

    let binary_tree = BinaryTree {
        head: Some(stack.pop().unwrap().1),
    };

    Ok(binary_tree)
}

fn part_1(input: &str) -> usize {
    unimplemented!();
}

fn part_2(input: &str) -> usize {
    unimplemented!();
}

fn main() {
    let part_1_result = part_1(INPUT_FILE);
    println!("[INFO]: Part 1: {:?}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_snailfish_numbers_1() {
        assert_eq!(
            parse("[1,2]").unwrap(),
            BinaryTree::new(Some(BTNode::new_pair(
                Some(BTNode::new_value(1)),
                Some(BTNode::new_value(2)),
            )))
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_2() {
        assert_eq!(
            parse("[[1,2],3]").unwrap(),
            BinaryTree::new(Some(BTNode::new_pair(
                Some(BTNode::new_pair(
                    Some(BTNode::new_value(1)),
                    Some(BTNode::new_value(2)),
                )),
                Some(BTNode::new_value(3))
            )))
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_3() {
        assert_eq!(
            parse("[9,[8,7]]").unwrap(),
            BinaryTree::new(Some(BTNode::new_pair(
                Some(BTNode::new_value(9)),
                Some(BTNode::new_pair(
                    Some(BTNode::new_value(8)),
                    Some(BTNode::new_value(7)),
                )),
            )))
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_4() {
        assert_eq!(
            parse("[[1,9],[8,5]]").unwrap(),
            BinaryTree::new(Some(BTNode::new_pair(
                Some(BTNode::new_pair(
                    Some(BTNode::new_value(1)),
                    Some(BTNode::new_value(9)),
                )),
                Some(BTNode::new_pair(
                    Some(BTNode::new_value(8)),
                    Some(BTNode::new_value(5)),
                )),
            )))
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_5() {
        assert_eq!(
            parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").unwrap(),
            BinaryTree::new(Some(BTNode::new_pair(
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
            )))
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_6() {
        assert!(parse("[1,2]").is_ok());
        assert!(parse("[[1,2],3]").is_ok());
        assert!(parse("[9,[8,7]]").is_ok());
        assert!(parse("[[1,9],[8,5]]").is_ok());
        assert!(parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").is_ok());
        assert!(parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]").is_ok());
        assert!(parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok());
    }

    #[test]
    fn it_adds_two_snailfish_numbers() {
        let a = parse("[1,2]").unwrap();
        let b = parse("[[3,4],5]").unwrap();

        assert_eq!(
            BinaryTree::addition(a, b),
            parse("[[1,2],[[3,4],5]]").unwrap()
        );
    }

    #[test]
    fn it_finds_explosive_pairs_1() {
        let binary_tree = parse("[[[[[9,8],1],2],3],4]").unwrap();
        let explosive_pair = BinaryTree::find_leftmost_explosive(&binary_tree.head.unwrap(), 0);
        assert!(explosive_pair.is_some());
        assert_eq!(explosive_pair, parse("[9,8]").unwrap().head);
    }

    #[test]
    fn it_finds_explosive_pairs_2() {
        let binary_tree = parse("[[[[1,[9,8]],2],3],4]").unwrap();
        let explosive_pair = BinaryTree::find_leftmost_explosive(&binary_tree.head.unwrap(), 0);
        assert_eq!(explosive_pair, parse("[9,8]").unwrap().head);
    }

    #[test]
    fn it_finds_explosive_pairs_3() {
        let binary_tree = parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let explosive_pair = BinaryTree::find_leftmost_explosive(&binary_tree.head.unwrap(), 0);
        assert_eq!(explosive_pair, parse("[3,2]").unwrap().head);
    }

    #[test]
    fn it_finds_explosive_pairs_4() {
        let binary_tree = parse("[[[[1,9],2],3],4]").unwrap();
        let explosive_pair = BinaryTree::find_leftmost_explosive(&binary_tree.head.unwrap(), 0);
        assert_eq!(explosive_pair, None);
    }

    #[test]
    fn it_explodes_examples_1() {
        let binary_tree = parse("[[[[[9,8],1],2],3],4]").unwrap();
        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, parse("[[[[0,9],2],3],4]").unwrap());
    }

    #[test]
    fn it_explodes_examples_2() {
        let binary_tree = parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, parse("[7,[6,[5,[7,0]]]]").unwrap());
    }

    #[test]
    fn it_explodes_examples_3() {
        let binary_tree = parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, parse("[[6,[5,[7,0]]],3]").unwrap());
    }

    #[test]
    fn it_explodes_examples_4() {
        let binary_tree = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(
            binary_tree,
            parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap()
        );
    }

    #[test]
    fn it_explodes_examples_5() {
        let binary_tree = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());
        assert_eq!(binary_tree, parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap());
    }

    #[test]
    fn it_handles_split_example() {
        let binary_tree = BinaryTree::addition(
            parse("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap(),
            parse("[1,1]").unwrap(),
        );
        assert_eq!(
            binary_tree,
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap()
        );

        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());

        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        BinaryTree::explode(&explosive_pair.unwrap());

        let splittable = BinaryTree::find_leftmost_splittable(binary_tree.head.as_ref().unwrap());
        BinaryTree::split(&splittable.unwrap());

        let splittable = BinaryTree::find_leftmost_splittable(binary_tree.head.as_ref().unwrap());
        BinaryTree::split(&splittable.unwrap());
        assert_eq!(binary_tree, parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap());

        let explosive_pair =
            BinaryTree::find_leftmost_explosive(binary_tree.head.as_ref().unwrap(), 0);
        println!("{:?}", explosive_pair);
        BinaryTree::explode(&explosive_pair.unwrap());

        assert_eq!(
            binary_tree,
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap()
        );
    }

    /*
    #[test]
    fn it_solves_part1_magnitude_examples() {
        assert_eq!(part_1("[9,1]"), 29);
        assert_eq!(part_1("[1,9]"), 21);
        assert_eq!(part_1("[[9,1],[1,9]]"), 129);
        assert_eq!(part_1("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(part_1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
        assert_eq!(part_1("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
        assert_eq!(part_1("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
        assert_eq!(part_1("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
        assert_eq!(
            part_1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            3488
        );
    }

    fn it_solves_part1_full_example() {
        assert_eq!(
            part_1(
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
            4140
        );
    }
    */
}
