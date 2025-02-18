// Time complexity: O(nm)
// Space complexity: O(m)
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let sum: i32 = rods.iter().sum();
        let mut dp = vec![-1; (sum + 1) as usize];
        dp[0] = 0;

        for &h in &rods {
            let prev = dp.clone();
            for i in 0..=(sum - h) {
                if prev[i as usize] < 0 {
                    continue;
                }
                dp[i as usize] = dp[i as usize].max(prev[i as usize]);
                dp[(i + h) as usize] = dp[(i + h) as usize].max(prev[i as usize]);
                dp[(i - h).abs() as usize] =
                    dp[(i - h).abs() as usize].max(prev[i as usize] + i.min(h));
            }
        }

        dp[0]
    }
}
