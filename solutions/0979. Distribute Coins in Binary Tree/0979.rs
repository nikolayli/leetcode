// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            node.as_ref().map_or(0, |n| {
                let n = n.borrow();
                let l = dfs(&n.left, ans);
                let r = dfs(&n.right, ans);
                *ans = l.abs() + r.abs();

                n.val - 1 + l + r
            })
        }

        let mut ans = 0;
        dfs(&root, &mut ans);

        ans
    }
}
