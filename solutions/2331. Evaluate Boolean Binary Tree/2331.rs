// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node_ref) => {
                let node = node_ref.borrow();
                match node.val {
                    0 => false,
                    1 => true,
                    2 => {
                        Self::evaluate_tree(node.left.clone())
                            || Self::evaluate_tree(node.right.clone())
                    }
                    _ => {
                        Self::evaluate_tree(node.left.clone())
                            && Self::evaluate_tree(node.right.clone())
                    }
                }
            }
            None => false,
        }
    }
}
