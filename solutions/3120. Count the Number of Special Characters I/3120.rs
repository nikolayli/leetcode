// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lowercase: [bool; 26] = [false; 26];
        let mut uppercase: [bool; 26] = [false; 26];
        let mut ans = 0;

        for c in word.chars() {
            if c.is_ascii_lowercase() {
                lowercase[c as usize - 'a' as usize] = true;
            } else {
                uppercase[c as usize - 'A' as usize] = true;
            }
        }

        for i in 0..26 {
            if lowercase[i] && uppercase[i] {
                ans += 1;
            }
        }

        ans
    }
}
