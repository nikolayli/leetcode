// Time complexity: O(logn)
// Space complexity: O(nlogn)
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![n; (n + 1) as usize];
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..=n {
            for j in 1..=i {
                if j * j > i {
                    break;
                }
                dp[i as usize] = dp[i as usize].min(dp[(i - j * j) as usize] + 1);
            }
        }

        dp[n as usize]
    }
}
