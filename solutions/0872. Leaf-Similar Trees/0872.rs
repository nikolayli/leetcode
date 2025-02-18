// Time Complexity: O(T1+T2)
// Space Complexity: O(T1+T2)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(node) = root {
                let mut result = Vec::new();
                let node_ref = node.borrow();
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    result.push(node_ref.val);
                }
                result.extend(dfs(node_ref.left.clone()));
                result.extend(dfs(node_ref.right.clone()));
                result
            } else {
                Vec::new()
            }
        }

        let leaves1 = dfs(root1);
        let leaves2 = dfs(root2);
        leaves1 == leaves2
    }
}
