// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix = 0;
        let mut prefix_to_index = HashMap::new();
        prefix_to_index.insert(0, -1);

        for (i, &num) in nums.iter().enumerate() {
            prefix += num;
            if k != 0 {
                prefix %= k;
            }
            if let Some(&index) = prefix_to_index.get(&prefix) {
                if i as i32 - index > 1 {
                    return true;
                }
            } else {
                prefix_to_index.insert(prefix, i as i32);
            }
        }

        false
    }
}
