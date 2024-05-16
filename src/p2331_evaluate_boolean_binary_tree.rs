#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        return match node.borrow().val {
            0 => false,
            1 => true,
            2 => evaluate_tree(node.borrow().left.clone()) || evaluate_tree(node.borrow().right.clone()),
            3 => evaluate_tree(node.borrow().left.clone()) && evaluate_tree(node.borrow().right.clone()),
            _ => unreachable!()
        };
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p2331_evaluate_boolean_binary_tree::evaluate_tree;

    #[test]
    fn test_one() {
        let tree = build_tree("[2,1,3,null,null,0,1]");
        let expected = true;
        let output = evaluate_tree(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[0]");
        let expected = false;
        let output = evaluate_tree(tree);
        assert_eq!(expected, output);
    }
}