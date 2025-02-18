// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        s.repeat(2)[1..s.len() * 2 - 1].find(&s).is_some()
    }
}
