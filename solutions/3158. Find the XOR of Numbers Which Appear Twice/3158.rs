// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for (&num, &freq) in count.iter() {
            if freq == 2 {
                ans ^= num;
            }
        }

        ans
    }
}
