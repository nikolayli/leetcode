// Time complexity: O(n^2)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let dictionary_set: HashSet<String> = dictionary.into_iter().collect();
        let mut dp = vec![n as i32; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for j in 0..i {
                let substring = &s[j..i];
                if dictionary_set.contains(substring) {
                    dp[i] = dp[i].min(dp[j]);
                } else {
                    dp[i] = dp[i].min(dp[j] + (i - j) as i32);
                }
            }
        }

        dp[n]
    }
}
