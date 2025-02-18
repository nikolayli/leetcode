// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut ans: i64 = 0;
        let mut sum: i64 = 0;
        let mut distinct: usize = 0;
        let mut count: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            let entry = count.entry(nums[i]).or_insert(0);
            *entry += 1;
            if *entry == 1 {
                distinct += 1;
            }
            if i >= k {
                let entry = count.entry(nums[i - k]).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    distinct -= 1;
                }
                sum -= nums[i - k] as i64;
            }
            if i >= k - 1 && distinct == k {
                ans = ans.max(sum);
            }
        }

        ans
    }
}
