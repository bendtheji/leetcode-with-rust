#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut moves = 0;
    dfs(root, &mut moves);
    moves
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
    if let Some(node) = root {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        let left_coins = dfs(left, moves);
        let right_coins = dfs(right, moves);

        *moves += left_coins.abs() + right_coins.abs();

        return node.borrow().val - 1 + left_coins + right_coins;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::bst_util::build_tree;
    use crate::p979_distribute_coins_in_binary_tree::distribute_coins;

    #[test]
    fn test_one() {
        let root = build_tree("[3,0,0]");
        let expected = 2;
        let output = distribute_coins(root);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let root = build_tree("[0,3,0]");
        let expected = 3;
        let output = distribute_coins(root);
        assert_eq!(expected, output);
    }
}