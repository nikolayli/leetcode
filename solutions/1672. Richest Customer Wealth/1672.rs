// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap_or(0)
    }
}
