// Time complexity: O(n*d^2)
// Space complexity: O(h)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: usize, ans: &mut i32) -> Vec<i32> {
            let mut d = vec![0; distance + 1];

            if let Some(root) = root {
                let root = root.borrow();
                if root.left.is_none() && root.right.is_none() {
                    d[0] = 1;
                    return d;
                }
                let dl = dfs(root.left.clone(), distance, ans);
                let dr = dfs(root.right.clone(), distance, ans);
                for i in 0..distance {
                    for j in 0..distance {
                        if i + j + 2 <= distance {
                            *ans += dl[i] * dr[j];
                        }
                    }
                }
                for i in 0..distance {
                    d[i + 1] = dl[i] + dr[i];
                }
            }

            d
        }
        let mut ans = 0;
        dfs(root, distance as usize, &mut ans);

        ans
    }
}
