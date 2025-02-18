// Time complexity: O(n+m)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels_set: HashSet<char> = jewels.chars().collect();
        stones
            .chars()
            .filter(|&stone| jewels_set.contains(&stone))
            .count() as i32
    }
}
