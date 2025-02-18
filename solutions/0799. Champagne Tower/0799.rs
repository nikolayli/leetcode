// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        let mut dp = vec![0.0; query_row + 1];
        dp[0] = poured as f64;

        for i in 0..query_row {
            let mut new_dp = vec![0.0; query_row + 1];
            for j in 0..=i {
                if dp[j] > 1.0 {
                    let overflow = (dp[j] - 1.0) / 2.0;
                    new_dp[j] += overflow;
                    new_dp[j + 1] += overflow;
                }
            }
            dp = new_dp;
        }

        dp[query_glass].min(1.0)
    }
}
