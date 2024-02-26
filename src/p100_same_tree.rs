#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let root_p = x.borrow();
            let root_q = y.borrow();
            root_p.val == root_q.val && is_same_tree(root_p.left.clone(), root_q.left.clone())
                && is_same_tree(root_p.right.clone(), root_q.right.clone())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p100_same_tree::is_same_tree;

    #[test]
    fn test_one() {
        let left = build_tree("[1,2,3]");
        let right = build_tree("[1,2,3]");
        assert!(is_same_tree(left, right));
    }

    #[test]
    #[should_panic]
    fn test_two() {
        let left = build_tree("[1,2]");
        let right = build_tree("[1,null,2]");
        assert!(is_same_tree(left, right));
    }

    #[test]
    #[should_panic]
    fn test_three() {
        let left = build_tree("[1,2,1]");
        let right = build_tree("[1,1,2]");
        assert!(is_same_tree(left, right));
    }
}