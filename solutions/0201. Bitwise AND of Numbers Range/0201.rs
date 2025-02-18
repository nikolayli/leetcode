// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut shift_bits = 0;

        while left != right {
            left >>= 1;
            right >>= 1;
            shift_bits += 1;
        }

        left << shift_bits
    }
}
