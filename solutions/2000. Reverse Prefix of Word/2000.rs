// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars[..=word.find(ch).unwrap_or(0)].reverse();

        char.into_iter().collect()
    }
}
