// Time complexity: O(n^2)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let k_mod = 1_000_000_007;
        let mut dp = vec![1; arr.len()];
        let mut num_to_index: HashMap<i32, usize> = HashMap::new();

        arr.sort_unstable();

        for (i, &root) in arr.iter().enumerate() {
            for j in 0..i {
                if root % arr[j] == 0 {
                    let right = root / arr[j];
                    if let Some(&index) = num_to_index.get(&right) {
                        dp[i] += dp[j] * dp[index];
                        dp[i] %= k_mod;
                    }
                }
            }
            num_to_index.insert(root, i);
        }

        (dp.iter().fold(0, |sum, &e| sum + e as i64) % k_mod) as i32
    }
}
