const INPUT_FILE: &str = include_str!("../../inputs/day18.txt");

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Pair,
    Value(u32),
}

#[derive(Debug, PartialEq, Clone)]
struct BTNode {
    left: Option<Box<BTNode>>,
    right: Option<Box<BTNode>>,
    op: Op,
}

impl BTNode {
    fn new_empty_pair() -> BTNode {
        BTNode {
            left: None,
            right: None,
            op: Op::Pair,
        }
    }
    fn new_value(value: u32) -> BTNode {
        BTNode {
            left: None,
            right: None,
            op: Op::Value(value),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BinaryTree {
    head: BTNode,
}

fn parse(input: &str) -> Result<BinaryTree, String> {
    let mut depth = 0;

    let mut stack: Vec<(usize, BTNode)> = Vec::new();

    for c in input.chars() {
        if c == '[' {
            stack.push((depth, BTNode::new_empty_pair()));
            depth += 1;
        } else if c == ']' {
            // Empty pair "[]" is illegal
            let (right_depth, right) = stack.pop().ok_or("stack pop")?;
            let (left_depth, left) = stack.pop().ok_or("stack pop")?;

            let node_and_depth = match left {
                BTNode {
                    op: Op::Value(_), ..
                } => {
                    let (depth, mut node) = stack.pop().ok_or("stack pop")?;
                    node.right = Some(Box::new(right));
                    node.left = Some(Box::new(left));
                    (depth, node)
                }
                BTNode {
                    op: Op::Pair,
                    ..
                } => {
                    if left_depth == right_depth {
                        // Same depth, so this was a left node and the
                        // pair node wrapping this is next
                        let (depth, mut node) = stack.pop().ok_or("stack pop")?;
                        node.right = Some(Box::new(right));
                        node.left = Some(Box::new(left));
                        (depth, node)
                    } else {
                        let mut node = left;
                        node.right = Some(Box::new(right));
                        (depth, node)
                    }
                }
            };

            // Now push the node back to stack
            stack.push(node_and_depth);
            depth -= 1;
        } else if c.is_digit(10) {
            let value = c.to_digit(10).ok_or("to_digit parsing")?;
            stack.push((depth, BTNode::new_value(value)));
        }
    }

    // Now the stack should only contain the head
    assert_eq!(stack.len(), 1);

    let binary_tree = BinaryTree {
        head: stack.pop().unwrap().1
    };

    Ok(binary_tree)
}

fn addition(a: BinaryTree, b: BinaryTree) {
    let mut new_head = BTNode::new_empty_pair();

    new_head.left = Some(Box::new(a.head));
    new_head.right = Some(Box::new(b.head));
    
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
            BinaryTree {
                head: BTNode {
                    left: Some(Box::new(BTNode::new_value(1))),
                    right: Some(Box::new(BTNode::new_value(2))),
                    op: Op::Pair,
                }
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_2() {
        assert_eq!(
            parse("[[1,2],3]").unwrap(),
            BinaryTree {
                head: BTNode {
                    left: Some(Box::new(BTNode {
                        left: Some(Box::new(BTNode::new_value(1))),
                        right: Some(Box::new(BTNode::new_value(2))),
                        op: Op::Pair,
                    })),
                    right: Some(Box::new(BTNode::new_value(3))),
                    op: Op::Pair,
                },
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_3() {
        assert_eq!(
            parse("[9,[8,7]]").unwrap(),
            BinaryTree {
                head: BTNode {
                    left: Some(Box::new(BTNode::new_value(9))),
                    right: Some(Box::new(BTNode {
                        left: Some(Box::new(BTNode::new_value(8))),
                        right: Some(Box::new(BTNode::new_value(7))),
                        op: Op::Pair,
                    })),
                    op: Op::Pair,
                },
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_4() {
        assert_eq!(
            parse("[[1,9],[8,5]]").unwrap(),
            BinaryTree {
                head: BTNode {
                    left: Some(Box::new(BTNode {
                        left: Some(Box::new(BTNode::new_value(1))),
                        right: Some(Box::new(BTNode::new_value(9))),
                        op: Op::Pair,
                    })),
                    right: Some(Box::new(BTNode {
                        left: Some(Box::new(BTNode::new_value(8))),
                        right: Some(Box::new(BTNode::new_value(5))),
                        op: Op::Pair,
                    })),
                    op: Op::Pair,
                },
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_5() {
        assert_eq!(
            parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").unwrap(),
            BinaryTree {
                head: BTNode {
                    left: Some(Box::new(BTNode {
                        left: Some(Box::new(BTNode {
                            left: Some(Box::new(BTNode {
                                left: Some(Box::new(BTNode::new_value(1))),
                                right: Some(Box::new(BTNode::new_value(2))),
                                op: Op::Pair,
                            })),
                            right: Some(Box::new(BTNode {
                                left: Some(Box::new(BTNode::new_value(3))),
                                right: Some(Box::new(BTNode::new_value(4))),
                                op: Op::Pair,
                            })),
                            op: Op::Pair,
                        })),
                        right: Some(Box::new(BTNode {
                            left: Some(Box::new(BTNode {
                                left: Some(Box::new(BTNode::new_value(5))),
                                right: Some(Box::new(BTNode::new_value(6))),
                                op: Op::Pair,
                            })),
                            right: Some(Box::new(BTNode {
                                left: Some(Box::new(BTNode::new_value(7))),
                                right: Some(Box::new(BTNode::new_value(8))),
                                op: Op::Pair,
                            })),
                            op: Op::Pair,
                        })),
                        op: Op::Pair,
                    })),
                    right: Some(Box::new(BTNode::new_value(9))),
                    op: Op::Pair,
                },
            }
        );
    }

    #[test]
    fn it_parses_snailfish_numbers_6() {
        assert!(parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]").is_ok());
        assert!(parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok());
    }

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
}
