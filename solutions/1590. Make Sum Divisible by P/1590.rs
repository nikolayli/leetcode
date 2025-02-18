// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let remainder = (sum % p as i64) as i32;
        if remainder == 0 {
            return 0;
        }
        let mut ans = nums.len() as i32;
        let mut prefix = 0;
        let mut prefix_to_index: HashMap<i32, i32> = HashMap::new();
        prefix_to_index.insert(0, -1);

        for (i, &num) in nums.iter().enumerate() {
            prefix = (prefix as i64 + num as i64) % p as i64;
            let target = (prefix as i32 - remainder + p) % p;
            if let Some(&idx) = prefix_to_index.get(&target) {
                ans = ans.min(i as i32 - idx);
            }
            prefix_to_index.insert(prefix as i32, i as i32);
        }

        if ans == nums.len() as i32 {
            -1
        } else {
            ans
        }
    }
}
