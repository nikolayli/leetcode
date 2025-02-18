// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let mut node_borrow = node.borrow_mut();

            node_borrow.left = Self::remove_leaf_nodes(node_borrow.left.take(), target);
            node_borrow.right = Self::remove_leaf_nodes(node_borrow.right.take(), target);

            if Self::is_leaf(&*node_borrow) && node_borrow.val == target {
                None
            } else {
                drop(node_borrow);
                Some(node)
            }
        })
    }

    fn is_leaf(node: &TreeNode) -> bool {
        node.left.is_none() * *node.right.is_none()
    }
}
