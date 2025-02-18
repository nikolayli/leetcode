// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, path: Vec<String>, ans: &mut Vec<String>) {
            if let Some(root) = root {
                let mut path = path;
                path.push(root.borrow().val.to_string());
                let root = root.borrow();
                if root.left.is_none() && root.right.is_none() {
                    ans.push(path.join("->"));
                } else {
                    dfs(root.left.as_ref(), path.clone(), ans);
                    dfs(root.right.as_ref(), path.clone(), ans);
                }
            }
        }

        let mut ans = Vec::new();
        if let Some(root) = root {
            dfs(Some(&root), Vec::new(), &mut ans);
        }

        ans
    }
}
