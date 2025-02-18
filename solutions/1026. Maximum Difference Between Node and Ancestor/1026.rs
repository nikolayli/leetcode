// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MAX, i32::MIN)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mini: i32, maxi: i32) -> i32 {
        match root {
            None => maxi - mini,
            Some(n) => {
                let node = n.borrow();
                [&node.left, &node.right]
                    .into_iter()
                    .map(|n| Self::dfs(n, mini.min(node.val), maxi.max(node.val)))
                    .max()
                    .unwrap_or(0)
            }
        }
    }
}
