// Time complexity: O(sort)
// Space complexity: O(sort)
use std::cmp::max;

impl Solution {
    pub fn min_operations_to_make_median_k(mut nums: Vec<i32>, k: i32) -> i64 {
        let mut ans: i64 = 0;

        nums.sort_unstable();
        for i in 0..=nums.len() / 2 {
            ans += max(0, nums[i] - k) as i64;
        }
        for i in nums.len() / 2..nums.len() {
            ans += max(0, k - nums[i]) as i64;
        }

        ans
    }
}
