// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let n = cost.len();

        for i in 2..n {
            cost[i] += cost[i - 1].min(cost[i - 2]);
        }

        cost[n - 1].min(cost[n - 2])
    }
}
