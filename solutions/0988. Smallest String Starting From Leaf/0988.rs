// Time complexity: O(nh)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();
        Solution::dfs(&root, String::new(), &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut path: String, ans: &mut String) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            let ch = char::from_u32((node_ref.val as u32) + ('a' as u32)).unwrap();
            path.insert(0, ch);

            if node_ref.left.is_none() && node_ref.right.is_none() {
                if ans.is_empty() || *ans > path {
                    *ans = path.clone();
                }
            }

            Self::dfs(&node_ref.left, path.clone(), ans);
            Self::dfs(&node_ref.right, path.clone(), ans);
        }
    }
}
