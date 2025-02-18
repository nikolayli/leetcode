// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        let mut ans = 0;

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for (_, &value) in &count {
            max_count = max_count.max(value)
        }

        for (_, &value) in &count {
            if value == max_count {
                ans += value;
            }
        }

        ans
    }
}
