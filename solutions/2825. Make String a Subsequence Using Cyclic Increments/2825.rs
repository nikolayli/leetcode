// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut i = 0;
        let str2_chars: Vec<char> = str2.chars().collect();

        for c in str1.chars() {
            if i < str2_chars.len()
                && (c == str2_chars[i] || (c as u8 + 1 - b'a') % 26 + b'a' == str2_chars[i] as u8)
            {
                i += 1;
                if i == str2_chars.len() {
                    return true;
                }
            }
        }

        false
    }
}
