// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut i = s.len() as i32 - 1;
        while i >= 0 && s.chars().nth(i as usize) == Some(' ') {
            i -= 1;
        }

        let last_index = i;
        while i >= 0 && s.chars().nth(i as usize) != Some(' ') {
            i -= 1;
        }

        (last_index - i) as i32
    }
}
