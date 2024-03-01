#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    } else {
        return false;
    }

    let mut level = 0;
    while !queue.is_empty() {
        let no_of_children = queue.len();
        let is_even = level % 2 == 0;
        let mut previous = match is_even {
            true => 0,
            false => i32::MAX
        };
        for _ in 0..no_of_children {
            let node = queue.pop_front().unwrap();
            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            match is_even {
                true => {
                    if val % 2 == 0 || val <= previous { return false; } else { previous = val; }
                }
                false => {
                    if val % 2 != 0 || val >= previous { return false; } else { previous = val; }
                }
            }

            if let Some(left) = left { queue.push_back(left); }
            if let Some(right) = right { queue.push_back(right); }
        }
        level += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p1609_even_odd_tree::is_even_odd_tree;

    #[test]
    fn test_one() {
        let tree = build_tree("[1,10,4,3,null,7,9,12,8,6,null,null,2]");
        let expected = true;
        let output = is_even_odd_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[5,4,2,3,3,7]");
        let expected = false;
        let output = is_even_odd_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let tree = build_tree("[5,9,1,3,5,7]");
        let expected = false;
        let output = is_even_odd_tree(tree);
        assert_eq!(expected, output);
    }
}