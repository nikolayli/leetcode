// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node_ref = node.borrow();
                let left = node_ref.left.clone();
                let right = node_ref.right.clone();
                let mut ans = 0;

                if let Some(left_node) = left {
                    let left_node_ref = left_node.borrow();
                    if left_node_ref.left.is_none() && left_node_ref.right.is_none() {
                        ans += left_node_ref.val;
                    } else {
                        ans += Self::sum_of_left_leaves(Some(left_node.clone()));
                    }
                }

                ans += Self::sum_of_left_leaves(right);

                ans
            }
            None => 0,
        }
    }
}
