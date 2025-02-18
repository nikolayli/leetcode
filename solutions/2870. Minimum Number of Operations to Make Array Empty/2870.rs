// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        if count.values().any(|&freq| freq == 1) {
            return -1;
        }

        let ans: i32 = count.values().map(|&freq| (freq + 2) / 3).sum();

        ans
    }
}
