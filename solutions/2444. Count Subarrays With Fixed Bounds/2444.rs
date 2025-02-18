// Time complexity: O(n)
// Space complexity: O(1)
use std::cmp;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut j: i64 = -1;
        let mut prev_min_k_index: i64 = -1;
        let mut prev_max_k_index: i64 = -1;

        for (i, &num) in nums.iter().enumerate() {
            if num < min_k || num > max_k {
                j = i as i64;
            }
            if num == min_k {
                prev_min_k_index = i as i64;
            }
            if num == max_k {
                prev_max_k_index = i as i64;
            }

            ans += cmp::max(0, cmp::min(prev_min_k_index, prev_max_k_index) - j);
        }

        ans
    }
}
