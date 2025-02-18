// Time complexity: O(nk)
// Space complexity: O(nk)
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 0..=n {
            dp[i][0] = 1;
        }

        for i in 1..=n {
            for j in 1..=k {
                dp[i][j] = (dp[i][j - 1] + dp[i - 1][j]) % MOD;
                if j >= i {
                    dp[i][j] = (dp[i][j] - dp[i - 1][j - 1] + MOD) % MOD;
                }
            }
        }

        dp[n][k]
    }
}
