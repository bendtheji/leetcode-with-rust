#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut queue = VecDeque::new();
    let mut result = vec![];
    if let Some(node) = root {
        queue.push_back(node.clone())
    }

    while !queue.is_empty() {
        let no_of_children = queue.len();
        let mut sum = 0.0;
        for _ in 0..no_of_children {
            if let Some(node) = queue.pop_front() {
                sum = sum + (node.borrow().val as f64);
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if let Some(left) = left { queue.push_back(left); }
                if let Some(right) = right { queue.push_back(right); }
            }
        }
        sum = sum / (no_of_children as f64);
        result.push(sum);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p637_average_of_levels_in_binary_tree::average_of_levels;

    #[test]
    fn test_one() {
        let tree = build_tree("[3,9,20,null,null,15,7]");
        let expected = vec![3.00000,14.50000,11.00000];
        let output = average_of_levels(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[3,9,20,15,7]");
        let expected = vec![3.00000,14.50000,11.00000];
        let output = average_of_levels(tree);
        assert_eq!(expected, output);
    }
}