// Time complexity: O(nlogn+mlogm)
// Space complexity: O(n)
impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut ans = 0;
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();

        jobs.sort_unstable();
        worker.sort_unstable();

        let mut i = 0;
        let mut max_profit = 0;

        for &w in &worker {
            while i < jobs.len() && w >= jobs[i].0 {
                max_profit = max_profit.max(jobs[i].1);
                i += 1;
            }
            ans += max_profit;
        }

        ans
    }
}
