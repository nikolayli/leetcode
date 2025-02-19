// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            match num_to_index.get(&(target - num)) {
                Some(&index) => return vec![index as i32, i as i32],
                None => num_to_index.insert(num, i as i32),
            };
        }
        panic!();
    }
}
