#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn remove_leaf_nodes(mut root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if should_remove_node(&mut root, target) {
        return None;
    }
    root
}


pub fn should_remove_node(root: &mut Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
    if let Some(node) = root {
        let node_val = node.borrow().val;
        let left = &mut node.borrow().left.clone();
        let right = &mut node.borrow().right.clone();

        if should_remove_node(left, target) {
            node.borrow_mut().left = None;
        }

        if should_remove_node(right, target) {
            node.borrow_mut().right = None;
        }

        return node_val == target && node.borrow().left == None && node.borrow().right == None;
    }
    true
}
// at current node
// three conditions to check if to remove
// left child is None
// right child is None
// val is target

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p1325_delete_leaves_with_a_given_value::remove_leaf_nodes;

    #[test]
    fn test_one() {
        let root = build_tree("[1,2,3,2,null,2,4]");
        let target = 2;
        let expected = build_tree("[1,null,3,null,4]");
        let output = remove_leaf_nodes(root, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let root = build_tree("[1,3,3,3,2]");
        let target = 3;
        let expected = build_tree("[1,3,null,null,2]");
        let output = remove_leaf_nodes(root, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let root = build_tree("[1,2,null,2,null,2]");
        let target = 2;
        let expected = build_tree("[1]");
        let output = remove_leaf_nodes(root, target);
        assert_eq!(expected, output);
    }
}