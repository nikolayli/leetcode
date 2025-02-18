// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node_borrow = node.borrow();
            let left = node_borrow.left.clone();
            let right = node_borrow.right.clone();

            if left.is_none() && right.is_none() {
                return node_borrow.val == target_sum;
            } else {
                return Solution::has_path_sum(left, target_sum - node_borrow.val)
                    || Solution::has_path_sum(right, target_sum - node_borrow.val);
            }
        }

        false
    }
}
