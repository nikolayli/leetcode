// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = HashSet::new();
        let mut ans = Vec::new();

        for &num in &nums {
            if seen.contains(&num) {
                ans.push(num);
            } else {
                seen.insert(num);
            }
        }

        ans
    }
}
