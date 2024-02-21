#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.as_ref().unwrap().borrow().val;
    let q = q.as_ref().unwrap().borrow().val;
    let mut ans = None;
    traverse(&root, p, q, &mut ans);
    ans
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32, ans: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        let is_root = (node.borrow().val == p) || (node.borrow().val == q);
        let is_left = traverse(&node.borrow().left, p, q, ans);
        let is_right = traverse(&node.borrow().right, p, q, ans);

        if is_root && is_left || is_root && is_right || is_left && is_right {
            *ans = Some(node.clone());
        }

        is_root || is_left || is_right
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::bst_util::{build_tree, find_node, TreeNode};
    use crate::p236_lca_of_bst::lowest_common_ancestor;

    #[test]
    fn test_one() {
        let tree = build_tree("[3,5,1,6,2,0,8,null,null,7,4]");
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let expected = find_node(&tree, 3);
        let output = lowest_common_ancestor(tree, p, q);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[3,5,1,6,2,0,8,null,null,7,4]");
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let expected = find_node(&tree, 5);
        let output = lowest_common_ancestor(tree, p, q);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let tree = build_tree("[1,2]");
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let expected = find_node(&tree, 1);
        let output = lowest_common_ancestor(tree, p, q);
        assert_eq!(expected, output);
    }
}
