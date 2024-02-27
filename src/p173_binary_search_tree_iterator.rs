use std::cell::RefCell;
use std::rc::Rc;

use crate::bst_util::TreeNode;

struct BSTIterator {
    nodes: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];
        Self::traverse(root, &mut nodes);
        Self {
            nodes
        }
    }

    fn next(&mut self) -> i32 {
        match self.nodes.pop() {
            Some(node) => node.borrow().val,
            None => 0
        }
    }

    fn has_next(&self) -> bool {
        !self.nodes.is_empty()
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            Self::traverse(right, nodes);
            nodes.push(node.clone());
            Self::traverse(left, nodes);
        }
    }
}