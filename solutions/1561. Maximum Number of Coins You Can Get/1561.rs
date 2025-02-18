// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let n = piles.len() / 3;
        piles.iter().skip(n).step_by(2).sum()
    }
}
