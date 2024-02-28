#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    let mut ans = 0;

    traverse(root, &mut max_depth, 0, &mut ans);
    ans
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, max_depth: &mut i32, current_depth: i32, ans: &mut i32) {
    if let Some(node) = root {
        let current_depth = current_depth + 1;
        if current_depth > *max_depth {
            *max_depth = current_depth;
            *ans = node.borrow().val;
        }
        traverse(node.borrow().left.clone(), max_depth, current_depth + 1, ans);
        traverse(node.borrow().right.clone(), max_depth, current_depth + 1, ans);
    }
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p513_find_bottom_left_tree_value::find_bottom_left_value;

    #[test]
    fn test_one() {
        let tree = build_tree("[2,1,3]");
        let expected = 1;
        let output = find_bottom_left_value(tree);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[1,2,3,4,null,5,6,null,null,7]");
        let expected = 7;
        let output = find_bottom_left_value(tree);

        assert_eq!(expected, output);
    }
}