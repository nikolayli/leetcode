// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut ans = 0;
            let mut stack = VecDeque::new();
            stack.push_back(node.clone());

            while let Some(curr) = stack.pop_back() {
                if let Some(left) = &curr.borrow().left {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        ans += left.borrow().val;
                    } else {
                        stack.push_back(left.clone());
                    }
                }
                if let Some(right) = &curr.borrow().right {
                    stack.push_back(right.clone());
                }
            }
            ans
        } else {
            0
        }
    }
}
