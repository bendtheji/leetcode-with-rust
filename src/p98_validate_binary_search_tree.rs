#![allow(dead_code)]

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut invalid = false;
    traverse(root.clone(), &mut invalid);
    !invalid
}

fn traverse(curr: Option<Rc<RefCell<TreeNode>>>, invalid: &mut bool) -> (i32, i32) {
    if let Some(root) = curr {
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        let root_val = root.borrow().val;

        let (mut left_min, mut left_max) = (i32::MAX, i32::MIN);
        let (mut right_min, mut right_max) = (i32::MAX, i32::MIN);

        let left_valid = match left.clone() {
            Some(_) => {
                (left_min, left_max) = traverse(left.clone(), invalid);
                if left_max >= root_val { false } else { true }
            }
            None => true,
        };

        let right_valid = match right.clone() {
            Some(_) => {
                (right_min, right_max) = traverse(right.clone(), invalid);
                if right_min <= root_val { false } else { true }
            }
            None => true,
        };

        if !left_valid || !right_valid {
            *invalid = true;
        }

        let min = cmp::min(root_val, cmp::min(left_min, right_min));
        let max = cmp::max(root_val, cmp::max(right_max, left_max));

        return (min, max);
    } else {
        (i32::MAX, i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p98_validate_binary_search_tree::is_valid_bst;

    #[test]
    fn test_one() {
        let tree = build_tree("[2,1,3]");
        let expected = true;
        let output = is_valid_bst(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tree = build_tree("[5,1,6,null,null,3,7]");
        let expected = false;
        let output = is_valid_bst(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let tree = build_tree("[2,2,2]");
        let expected = false;
        let output = is_valid_bst(tree);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_four() {
        let tree = build_tree("[2147483647]");
        let expected = true;
        let output = is_valid_bst(tree);
        assert_eq!(expected, output);
    }
}