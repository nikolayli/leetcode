// Time complexity: O(n)
// Space complexity: O(n)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = vec![None; 10001];
        queue[0] = root;
        let (mut idx, mut queue_size, mut incr) = (0, 1, 0);
        let mut depth = vec![0; 10001];
        let mut depth_idx = 0;
        while idx < queue_size {
            if let Some(node) = queue[idx].clone() {
                depth[depth_idx] = node.borrow().val;
                depth_idx += 1;
                if node.borrow().left.is_some() {
                    queue[queue_size + incr] = node.borrow().left.clone();
                    incr += 1;
                }

                if node.borrow().right.is_some() {
                    queue[queue_size + incr] = node.borrow().right.clone();
                    incr += 1;
                }

                if idx + 1 == queue_size {
                    if incr > 0 {
                        depth_idx = 0;
                    }

                    queue_size += incr;
                    incr = 0;
                }

                idx += 1;
            }
        }

        depth[0]
    }
}
