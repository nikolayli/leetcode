// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut prev = cost[0];
        let mut curr = cost[1];

        for i in 2..cost.len() {
            let next = cost[i] + prev.min(curr);
            prev = curr;
            curr = next;
        }

        prev.min(curr)
    }
}
