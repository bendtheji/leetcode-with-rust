#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let root_node = node.borrow();

        let mut new_root = TreeNode::new(root_node.val);

        new_root.left = invert_tree(root_node.right.clone());
        new_root.right = invert_tree(root_node.left.clone());

        return Some(Rc::new(RefCell::new(new_root)));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p226_invert_binary_tree::invert_tree;

    #[test]
    fn test_one() {
        let tree = build_tree("[4,2,7,1,3,6,9]");
        let expected = build_tree("[4,7,2,9,6,3,1]");
        let output = invert_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[2,1,3]");
        let expected = build_tree("[2,3,1]");
        let output = invert_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let tree = None;
        let expected = None;
        let output = invert_tree(tree);
        assert_eq!(expected, output);
    }
}