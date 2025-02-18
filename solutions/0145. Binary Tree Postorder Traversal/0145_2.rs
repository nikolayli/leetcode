// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let None = root {
            return vec![];
        }

        let mut ans = vec![];
        let mut stack = vec![root.clone()];

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                if let Some(left) = node.borrow().left.clone() {
                    stack.push(Some(left));
                }
                if let Some(right) = node.borrow().right.clone() {
                    stack.push(Some(right));
                }
                ans.push(node.borrow().val);
            }
        }

        ans.into_iter().rev().collect()
    }
}
