// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sums = Vec::new();

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, level_sums: &mut Vec<i64>) {
            if let Some(ref n) = node {
                let n = n.borrow();
                if level == level_sums.len() {
                    level_sums.push(0);
                }
                level_sums[level] += n.val as i64;
                dfs(&n.left, level + 1, level_sums);
                dfs(&n.right, level + 1, level_sums);
            }
        }

        dfs(&root, 0, &mut level_sums);

        if level_sums.len() < k as usize {
            return -1;
        }

        let mut heap = BinaryHeap::new();
        for &sum in &level_sums {
            heap.push(Reverse(sum));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        if let Some(Reverse(ans)) = heap.pop() {
            ans
        } else {
            -1
        }
    }
}
