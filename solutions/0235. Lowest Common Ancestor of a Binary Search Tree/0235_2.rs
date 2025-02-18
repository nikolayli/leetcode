// Time complexity: O(h)
// Space complexity: O(1)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root {
            let node_ref = node.borrow();
            let node_val = node_ref.val;

            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;

            if p_val < node_val && q_val < node_val {
                root = node_ref.left.clone();
            } else if p_val > node_val && q_val > node_val {
                root = node_ref.right.clone();
            } else {
                return Some(node.clone());
            }
        }

        None
    }
}
