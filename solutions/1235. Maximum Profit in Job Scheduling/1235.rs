// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut data = Vec::with_capacity(n);

        for i in 0..n {
            data.push((start_time[i], end_time[i], profit[i]));
        }

        data.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut dp = vec![(0, 0)];

        for x in data {
            let p = match dp.binary_search(&(x.0 + 1, 0)) {
                Ok(i) => i,
                Err(i) => i - 1,
            };
            if dp[p].1 + x.2 > dp[dp.len() - 1].1 {
                dp.push((x.1, dp[p].1 + x.2));
            }
        }

        return dp[dp.len() - 1].1;
    }
}
