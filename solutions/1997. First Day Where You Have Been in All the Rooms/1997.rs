// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        const k_mod: i64 = 1_000_000_007;
        let n = next_visit.len();
        let mut dp = vec![0; n];

        for i in 1..n {
            dp[i] = (2 * dp[i - 1] - dp[next_visit[i - 1] as usize] + 2 + k_mod) % k_mod;
        }

        dp[n - 1] as i32
    }
}
