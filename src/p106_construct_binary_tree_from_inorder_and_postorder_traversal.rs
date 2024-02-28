use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_from_slice(&inorder[..], &postorder[..])
}

fn build_tree_from_slice(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = postorder.len();
    if n == 0 { return None; }
    let root = postorder[n - 1];
    let mut root_node = TreeNode::new(root);
    let root_index = inorder.iter().position(|&x| x == root).unwrap();

    let right_inorder = &inorder[root_index + 1..];
    let right_no_of_elems = right_inorder.len();
    let right_postorder = &postorder[n - 1 - right_no_of_elems..n - 1];

    let left_inorder = &inorder[0..root_index];
    let left_postorder = &postorder[0..n - 1 - right_no_of_elems];

    root_node.left = build_tree_from_slice(left_inorder, left_postorder);
    root_node.right = build_tree_from_slice(right_inorder, right_postorder);

    Some(Rc::new(RefCell::new(root_node)))
}

#[cfg(test)]
mod tests {
    use crate::p106_construct_binary_tree_from_inorder_and_postorder_traversal::build_tree;
    use crate::bst_util;

    #[test]
    fn test_one() {
        let inorder = vec![9,3,15,20,7];
        let postorder = vec![9,15,7,20,3];

        let expected = build_tree(inorder, postorder);
        let output = bst_util::build_tree("[3,9,20,null,null,15,7]");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let inorder = vec![-1];
        let postorder = vec![-1];

        let expected = build_tree(inorder, postorder);
        let output = bst_util::build_tree("[-1]");

        assert_eq!(expected, output);
    }
}