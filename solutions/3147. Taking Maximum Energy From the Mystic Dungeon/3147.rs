// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = energy.to_vec();
        for i in (0..energy.len() - k).rev() {
            dp[i] += dp[i + k];
        }
        *dp.iter().max().unwrap()
    }
}
