// Time complexity: O(sort)
// Space complexity: O(n)
use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();

        nums.sort_unstable_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ba.cmp(&ab)
        });

        if nums[0] == "0" {
            return "0".to_string();
        }

        nums.concat()
    }
}
