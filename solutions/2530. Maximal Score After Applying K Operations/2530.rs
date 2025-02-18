// Time complexity:  O(nlog⁡n+klog⁡n)
// Space complexity: O(n)
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut max_heap = BinaryHeap::from(nums);

        for _ in 0..k {
            if let Some(num) = max_heap.pop() {
                ans += num as i64;
                max_heap.push((num + 2) / 3);
            }
        }

        ans
    }
}
