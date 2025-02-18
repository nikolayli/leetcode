// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            node.as_ref().map_or((0, 0, 0), |n| {
                let node = n.borrow();
                let (left_sum, left_count, left_ans) = dfs(&node.left);
                let (right_sum, right_count, right_ans) = dfs(&node.right);
                let sum = node.val + left_sum + right_sum;
                let count = 1 + left_count + right_count;
                let ans = left_ans + right_ans + if sum / count == node.val { 1 } else { 0 };
                (sum, count, ans)
            })
        }

        let (_, _, ans) = dfs(&root);
        ans
    }
}
