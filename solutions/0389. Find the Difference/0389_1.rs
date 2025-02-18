// Time complexity: O(n)
// Space complexity: O(1)
use std::ops::BitXor;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s_xors = s.bytes().fold(0, BitXor::bitxor);
        let t_xors = t.bytes().fold(0, BitXor::bitxor);
        (s_xors ^ t_xors) as char
    }
}
