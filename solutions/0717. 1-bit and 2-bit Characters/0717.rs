// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let mut i = 0;

        while i < n - 1 {
            if bits[i] == 0 {
                i += 1;
            } else {
                i += 2;
            }
        }

        i == n - 1
    }
}
