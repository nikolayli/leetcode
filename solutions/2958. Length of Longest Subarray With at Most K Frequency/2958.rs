// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();

        let mut l = 0;
        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            while count[&nums[r]] == k + 1 {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}
