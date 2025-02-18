// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ans = 0;

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for (&num, &freq) in &count {
            if let Some(&other_freq) = count.get(&(k - num)) {
                ans += freq.min(other_freq);
            }
        }

        ans / 2
    }
}
