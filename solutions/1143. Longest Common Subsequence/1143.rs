// Time complexity: O(nm)
// Space complexity: O(nm)
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                dp[i + 1][j + 1] = if text1.chars().nth(i) == text2.chars().nth(j) {
                    dp[i][j] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }

        dp[n][m]
    }
}
