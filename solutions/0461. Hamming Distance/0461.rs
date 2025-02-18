// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
