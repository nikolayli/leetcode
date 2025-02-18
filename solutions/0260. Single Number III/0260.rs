// Time Complexity: O(n)
// Space Complexity: O(1)
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xors = nums.iter().fold(0, |xor, &num| xor ^ num);
        let lowbit = xors & -xors;
        let mut ans = vec![0, 0];

        for &num in &nums {
            if num & lowbit != 0 {
                ans[0] ^= num;
            } else {
                ans[1] ^= num;
            }
        }

        ans
    }
}
