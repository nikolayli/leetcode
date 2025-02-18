// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vowelsCount = |s: &str| s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count();
        vowelsCount(&s[..s.len() / 2]) == vowelsCount(&s[s.len() / 2..])
    }
}
