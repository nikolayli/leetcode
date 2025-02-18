// Time complexity: O(n^3)
// Space complexity: O(n^2)
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![n as i32; n]; n];

        for i in 0..n {
            dp[i][i] = 1;
        }

        for j in 0..n {
            for i in (0..=j).rev() {
                for k in i..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k + 1][j] - if s[k] == s[j] { 1 } else { 0 });
                }
            }
        }

        dp[0][n - 1]
    }
}
