// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(root.as_ref(), &mut max);
        max
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::dfs(node.left.as_ref(), max);
            let right = Self::dfs(node.right.as_ref(), max);
            let left = if node
                .left
                .as_ref()
                .map_or(false, |n| n.borrow().val == node.val)
            {
                left + 1
            } else {
                0
            };
            let right = if node
                .right
                .as_ref()
                .map_or(false, |n| n.borrow().val == node.val)
            {
                right + 1
            } else {
                0
            };
            *max = (*max).max(left + right);
            left.max(right)
        } else {
            0
        }
    }
}
