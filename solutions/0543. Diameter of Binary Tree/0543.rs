// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(n) = node {
                let n_borrowed = n.borrow();
                let l = max_depth(&n_borrowed.left, ans);
                let r = max_depth(&n_borrowed.right, ans);
                *ans = (*ans).max(l + r);
                1 + l.max(r)
            } else {
                0
            }
        }

        let mut ans = 0;
        max_depth(&root, &mut ans);

        ans
    }
}
