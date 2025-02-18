// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        n * (n % 2 + 1)
    }
}
