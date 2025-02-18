// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![vec![0i64; 3]; 2];
        dp[0][0] = 1;

        for _ in 0..n {
            let prev = dp.clone();
            dp[0][0] = (prev[0][0] + prev[0][1] + prev[0][2]) % MOD;
            dp[0][1] = prev[0][0];
            dp[0][2] = prev[0][1];
            dp[1][0] =
                (prev[0][0] + prev[0][1] + prev[0][2] + prev[1][0] + prev[1][1] + prev[1][2]) % MOD;
            dp[1][1] = prev[1][0];
            dp[1][2] = prev[1][1];
        }

        dp.iter()
            .flat_map(|row| row.iter())
            .fold(0, |sum, &val| (sum + val) % MOD) as i32
    }
}
