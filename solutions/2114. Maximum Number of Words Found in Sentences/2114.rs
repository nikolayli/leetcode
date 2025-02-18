// Time complexity: O(nm)
// Space complexity: O(1)
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.chars().filter(|&c| c == ' ').count() as i32 + 1)
            .max()
            .unwrap_or(0)
    }
}
