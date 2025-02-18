// Time Complexity: O(n)
// Space Complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::inorder(root, None)
    }

    fn inorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => {
                let mut node = root.borrow_mut();
                let ans = Self::inorder(node.left.take(), Some(root.clone()));
                node.right = Self::inorder(node.right.take(), tail);

                ans
            }
            None => tail,
        }
    }
}
