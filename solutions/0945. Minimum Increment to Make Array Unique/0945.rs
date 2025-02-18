// Time complexity: O(sort)
// Space complexity: O(sort)
use std::cmp::max;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        nums.iter()
            .fold((0, 0), |(ans, min_available), &num| {
                let increment = max(min_available - num, 0);
                let new_min_available = max(min_available, num) + 1;
                (ans + increment, new_min_available)
            })
            .0
    }
}
