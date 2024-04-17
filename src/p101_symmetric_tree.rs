#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        return is_mirror(node.borrow().left.clone(), node.borrow().right.clone());
    }
    true
}

fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (Some(node_left), Some(node_right)) => {
            node_left.borrow().val == node_right.borrow().val
                && is_mirror(node_left.borrow().left.clone(), node_right.borrow().right.clone())
                && is_mirror(node_left.borrow().right.clone(), node_right.borrow().left.clone())
        }
        (None, None) => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p101_symmetric_tree::is_symmetric;

    #[test]
    fn test_one() {
        let root = build_tree("[1,2,2,3,4,4,3]");
        let expected = true;
        let output = is_symmetric(root);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let root = build_tree("[1,2,2,null,3,null,3]");
        let expected = false;
        let output = is_symmetric(root);
        assert_eq!(expected, output);
    }
}