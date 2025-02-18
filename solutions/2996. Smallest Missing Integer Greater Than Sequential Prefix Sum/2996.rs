// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut ans = nums[0];

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }
            ans += nums[i];
        }

        while nums_set.contains(&ans) {
            ans += 1;
        }

        ans
    }
}
