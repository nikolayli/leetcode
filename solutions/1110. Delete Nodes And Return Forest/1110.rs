// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = Vec::new();

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            is_root: bool,
            to_delete: &Vec<i32>,
            ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node_rc) = node {
                let mut node = node_rc.borrow_mut();
                let deleted = to_delete.contains(&node.val);

                if is_root && !deleted {
                    ans.push(Some(node_rc.clone()));
                }

                node.left = dfs(&node.left.take(), deleted, to_delete, ans);
                node.right = dfs(&node.right.take(), deleted, to_delete, ans);

                if deleted {
                    None
                } else {
                    Some(node_rc.clone())
                }
            } else {
                None
            }
        }

        dfs(&root, true, &to_delete, &mut ans);
        ans
    }
}
