// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        (0..24)
            .map(|i| candidates.iter().filter(|&&c| (c >> i) & 1 == 1).count() as i32)
            .max()
            .unwrap_or(0)
    }
}
