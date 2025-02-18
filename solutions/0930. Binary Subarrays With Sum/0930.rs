// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0;
        let mut prefix = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();
        count.insert(0, 1);

        for num in nums {
            prefix += num;
            ans += count.get(&(prefix - goal)).unwrap_or(&0);
            *count.entry(prefix).or_insert(0) += 1;
        }

        ans
    }
}
