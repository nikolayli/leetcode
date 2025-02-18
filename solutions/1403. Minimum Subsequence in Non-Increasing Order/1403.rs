// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut max_heap = BinaryHeap::from_iter(nums.iter().cloned());
        let half: i32 = nums.iter().sum::<i32>() / 2;
        let mut sum = 0;

        while sum <= half {
            if let Some(top) = max_heap.pop() {
                ans.push(top);
                sum += top;
            }
        }

        ans
    }
}
