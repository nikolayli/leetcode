// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut seen = HashSet::new();

        for num in nums {
            if seen.contains(&-num) {
                ans = ans.max()
            } else {
                seen.insert(num);
            }
        }

        ans
    }
}
