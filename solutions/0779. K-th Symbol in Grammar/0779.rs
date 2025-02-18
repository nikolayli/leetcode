// Time complexity: O(logk)
// Space complexity: O(1)
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let count = (k - 1).count_ones();
        if count % 2 == 0 {
            0
        } else {
            1
        }
    }
}
