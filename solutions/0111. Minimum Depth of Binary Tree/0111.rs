// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = Self::min_depth(node.borrow().left.clone());
                let right = Self::min_depth(node.borrow().right.clone());

                if left == 0 || right == 0 {
                    return left.max(right) + 1;
                }

                left.min(right) + 1
            }
            None => 0,
        }
    }
}
