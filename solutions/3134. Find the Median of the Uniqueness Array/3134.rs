// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_subarrays = n * (n + 1) / 2;
        let mut left = 1;
        let mut right = n as i32;

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::count_at_least_k_distinct(&nums, mid) * 2 >= total_subarrays as i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn count_at_least_k_distinct(nums: &[i32], k: i32) -> i64 {
        let mut count = 0;
        let mut left = 0;
        let mut freq = HashMap::new();

        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0) += 1;
            while freq.len() > k as usize {
                *freq.get_mut(&nums[left]).unwrap() -= 1;
                if freq[&nums[left]] == 0 {
                    freq.remove(&nums[left]);
                }
                left += 1;
            }
            count += (right - left + 1) as i64;
        }

        count
    }
}
