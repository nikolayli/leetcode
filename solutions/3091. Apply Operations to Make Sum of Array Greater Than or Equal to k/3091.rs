// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        let a = (k as f64).sqrt() as i32;
        a + (k - 1) / a - 1
    }
}
