// Time complexity: O(nm)
// Space complexity: O(n+m)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node_rc) => {
                let node = node_rc.borrow();
                Self::is_continuous_sub_path(&head, &Some(node_rc.clone()))
                    || Self::is_sub_path(head.clone(), node.left.clone())
                    || Self::is_sub_path(head, node.right.clone())
            }
            None => false,
        }
    }

    fn is_continuous_sub_path(
        head: &Option<Box<ListNode>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if head.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }

        let head = head.as_ref().unwrap();
        let root = root.as_ref().unwrap().borrow();

        head.val == root.val
            && (Self::is_continuous_sub_path(&head.next, &root.left)
                || Self::is_continuous_sub_path(&head.next, &root.right))
    }
}
