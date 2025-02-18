// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let mut i = 0;
        let mut j = 0;
        let s_len = s.len();
        let s_bytes = unsafe { s.as_bytes_mut() };

        while i < s_len {
            while i < j || (i < s_len && s_bytes[i] == b' ') {
                i += 1;
            }
            while j < i || (j < s_len && s_bytes[j] != b' ') {
                j += 1;
            }
            s_bytes[i..j].reverse();
        }

        s
    }
}
