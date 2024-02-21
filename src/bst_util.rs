#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Builds a LeetCode tree from a LeetCode tree string input
// e.g.:
// - input: "[5,3,6,2,4,null,null,1]" (without quotes)
// - output:
//            5
//           / \
//          3   6
//         / \
//        2   4
//       /
//      1
pub fn build_tree(nodes: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let nodes = nodes.trim(); // Did you copy-paste some spaces? this should get rid of them
    let nodes = &nodes[1..nodes.len() - 1]; // we only need the parts inside the brackets.
    let mut values: VecDeque<&str> = nodes
        .split(',')
        .map(|x| x.trim()) // Some more trimming never hurts...
        .collect();

    if values.is_empty() {
        return None;
    }

    let mut pointers = VecDeque::new();
    let root = TreeNode::new(values.pop_front().unwrap().parse::<i32>().unwrap());
    let root = Rc::new(RefCell::new(root));
    pointers.push_back((root.clone(), Side::Left));
    pointers.push_back((root.clone(), Side::Right));

    while let Some((node, side)) = pointers.pop_front() {
        match values.pop_front() {
            Some("null") => {}
            Some(other) => {
                let newnode = new_node(other.parse::<i32>().unwrap());
                pointers.push_back((newnode.clone(), Side::Left));
                pointers.push_back((newnode.clone(), Side::Right));
                match side {
                    Side::Left => (*node).borrow_mut().left = Some(newnode),
                    Side::Right => (*node).borrow_mut().right = Some(newnode),
                }
            }
            None => return Some(root),
        };
    }

    unreachable!()
}

fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    return Rc::new(RefCell::new(TreeNode::new(val)));
}

pub fn find_node(curr: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = curr {
        if node.borrow().val == val {
            return Some(node.clone())
        }
        match (find_node(&node.borrow().left, val), find_node(&node.borrow().right, val)){
            (Some(node), _) => Some(node),
            (_, Some(node)) => Some(node),
            (None, None) => None
        }
    }
    else {
        None
    }
}

enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// test