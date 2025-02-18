// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.chars()
            .enumerate()
            .take(s.len() - 1)
            .map(|(i, _)| (s.as_bytes()[i] as i32 - s.as_bytes()[i + 1] as i32).abs())
            .sum()
    }
}
