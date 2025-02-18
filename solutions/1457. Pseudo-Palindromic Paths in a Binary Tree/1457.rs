use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, path: i32, ans: &mut i32) {
            if let Some(node) = node {
                let node = node.borrow();
                let path = path ^ (1 << node.val);

                if node.left.is_none() && node.right.is_none() {
                    if path & (path - 1) == 0 {
                        *ans += 1;
                    }
                } else {
                    dfs(node.left.clone(), path, ans);
                    dfs(node.right.clone(), path, ans);
                }
            }
        }

        let mut ans = 0;
        dfs(root, 0, &mut ans);

        ans
    }
}
