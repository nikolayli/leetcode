// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val < low {
                return Self::range_sum_bst(node.right.clone(), low, high);
            } else if node.val > high {
                return Self::range_sum_bst(node.left.clone(), low, high);
            } else {
                return node.val
                    + Self::range_sum_bst(node.right.clone(), low, high)
                    + Self::range_sum_bst(node.left.clone(), low, high);
            }
        }

        0
    }
}
