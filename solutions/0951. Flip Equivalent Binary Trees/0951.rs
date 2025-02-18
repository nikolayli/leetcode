// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                node1.val == node2.val
                    && ((Self::flip_equiv(node1.left.clone(), node2.left.clone())
                        && Self::flip_equiv(node1.right.clone(), node2.right.clone()))
                        || (Self::flip_equiv(node1.left.clone(), node2.right.clone())
                            && Self::flip_equiv(node1.right.clone(), node2.left.clone())))
            }
            _ => false,
        }
    }
}
