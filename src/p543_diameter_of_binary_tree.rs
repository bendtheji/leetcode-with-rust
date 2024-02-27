#![allow(dead_code)]

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_length = 0;
    traverse(root, &mut max_length);
    max_length
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, max_length: &mut i32) -> i32 {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        let left_path = traverse(left, max_length);
        let right_path = traverse(right, max_length);

        *max_length = cmp::max(left_path + right_path, *max_length);

        return cmp::max(left_path, right_path) + 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p543_diameter_of_binary_tree::diameter_of_binary_tree;

    #[test]
    fn test_one() {
        let tree = build_tree("[1,2,3,4,5]");
        let expected = 3;
        let output = diameter_of_binary_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[1,2]");
        let expected = 1;
        let output = diameter_of_binary_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let tree = build_tree("[4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]");
        let expected = 8;
        let output = diameter_of_binary_tree(tree);
        assert_eq!(expected, output);
    }
}