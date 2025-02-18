// Time complexity: O(one*zero)
// Space complexity: O(one*zero)
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let k_mod = 1_000_000_007;
        let mut dp = vec![vec![vec![0; 2]; (one + 1) as usize]; (zero + 1) as usize];

        for i in 0..=zero {
            for j in 0..=one {
                dp[i as usize][j as usize][0] = 0;
                dp[i as usize][j as usize][1] = 0;

                if i == 0 || j == 0 {
                    if 0 < i && i <= limit {
                        dp[i as usize][j as usize][0] = 1;
                    }
                    if 0 < j && j <= limit {
                        dp[i as usize][j as usize][1] = 1;
                    }
                    continue;
                }

                if i > 0 {
                    dp[i as usize][j as usize][0] = (dp[i as usize - 1][j as usize][0]
                        + dp[i as usize - 1][j as usize][1])
                        % k_mod;
                }
                if i - limit > 0 {
                    dp[i as usize][j as usize][0] = (dp[i as usize][j as usize][0] + k_mod
                        - dp[(i - 1 - limit) as usize][j as usize][1])
                        % k_mod;
                }
                if j > 0 {
                    dp[i as usize][j as usize][1] = (dp[i as usize][j as usize - 1][0]
                        + dp[i as usize][j as usize - 1][1])
                        % k_mod;
                }
                if j - limit > 0 {
                    dp[i as usize][j as usize][1] = (dp[i as usize][j as usize][1] + k_mod
                        - dp[i as usize][(j - 1 - limit) as usize][0])
                        % k_mod;
                }
            }
        }

        dp[zero as usize][one as usize][0] =
            (dp[zero as usize][one as usize][0] + dp[zero as usize][one as usize][1]) % k_mod;

        dp[zero as usize][one as usize][0]
    }
}
