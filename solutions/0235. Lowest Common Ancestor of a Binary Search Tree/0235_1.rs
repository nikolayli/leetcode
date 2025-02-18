// Time complexity: O(h)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root, p, q) {
            (Some(r), Some(p), Some(q)) => {
                let r_borrow = r.borrow();
                let p_borrow = p.borrow();
                let q_borrow = q.borrow();

                if p_borrow.val < r_borrow.val && q_borrow.val < r_borrow.val {
                    return Self::lowest_common_ancestor(
                        r_borrow.left.clone(),
                        Some(p.clone()),
                        Some(q.clone()),
                    );
                }
                if p_borrow.val > r_borrow.val && q_borrow.val > r_borrow.val {
                    return Self::lowest_common_ancestor(
                        r_borrow.right.clone(),
                        Some(p.clone()),
                        Some(q.clone()),
                    );
                }
                Some(r.clone())
            }
            _ => None,
        }
    }
}
