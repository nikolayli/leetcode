// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let position_in_section = k & -k;
        let is_in_inverted_part = (k / position_in_section) >> 1 & 1 == 1;
        let original_bit_is_one = k & 1 == 0;

        if is_in_inverted_part {
            if original_bit_is_one {
                '0'
            } else {
                '1'
            }
        } else {
            if original_bit_is_one {
                '1'
            } else {
                '0'
            }
        }
    }
}
