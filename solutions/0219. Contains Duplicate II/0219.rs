// Time complexity: O(n)
// Space complexity: O(k)
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen = HashSet::new();

        for (i, &num) in nums.iter().enumerate() {
            if !seen.insert(num) {
                return true;
            }

            if i as i32 >= k {
                seen.remove(&(nums[i as usize - k as usize]));
            }
        }

        false
    }
}
