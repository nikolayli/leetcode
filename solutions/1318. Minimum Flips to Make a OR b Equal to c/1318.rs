// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        (((a | b) ^ c).count_ones() + ((a & b) & !c).count_ones()) as i32
    }
}
