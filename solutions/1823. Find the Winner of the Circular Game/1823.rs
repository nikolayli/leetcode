// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        (2..=n).fold(0, |ans, i| (ans + k) % i) + 1
    }
}
