// Time complexity: O(n)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut left_count = 0;
        let mut right_count = 0;

        fn count(
            root: Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            left_count: &mut i32,
            right_count: &mut i32,
        ) -> i32 {
            if let Some(n) = root {
                let l = count(n.borrow().left.clone(), x, left_count, right_count);
                let r = count(n.borrow().right.clone(), x, left_count, right_count);

                if n.borrow().val == x {
                    *left_count = l;
                    *right_count = r;
                }

                return 1 + l + r;
            }
            0
        }

        count(root.clone(), x, &mut left_count, &mut right_count);
        let remaining = n - left_count - right_count - 1;

        left_count > n / 2 || right_count > n / 2 || remaining > n / 2
    }
}
