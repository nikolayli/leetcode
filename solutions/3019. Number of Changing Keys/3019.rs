// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut ans = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for i in 1..s_chars.len() {
            if s_chars[i].to_ascii_lowercase() != s_chars[i - 1].to_ascii_lowercase() {
                ans += 1;
            }
        }
        ans
    }
}
