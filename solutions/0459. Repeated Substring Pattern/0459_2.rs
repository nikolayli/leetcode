// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        for period in 1..=s_chars.len() / 2 {
            if s_chars.len() % period == 0 {
                let mut good = true;
                for i in period..s_chars.len() {
                    if s_chars[i] != s_chars[i - period] {
                        good = false;
                        break;
                    }
                }
                if good {
                    return true;
                }
            }
        }

        false
    }
}
