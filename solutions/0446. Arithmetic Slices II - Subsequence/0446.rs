// Time complexity: O(n^2)
// Space complexity: O(n^2)
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut dp = vec![vec![0; n]; n];
        let mut num_to_indices: HashMap<i64, Vec<usize>> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            num_to_indices
                .entry(*num as i64)
                .or_insert(Vec::new())
                .push(i);
        }

        for i in 0..n {
            for j in 0..i {
                let target = nums[j] as i64 * 2 - nums[i] as i64;
                if let Some(indices) = num_to_indices.get(&target) {
                    for &k in indices {
                        if k < j {
                            dp[i][j] += dp[j][k] + 1;
                        }
                    }
                }

                ans += dp[i][j];
            }
        }

        ans
    }
}
