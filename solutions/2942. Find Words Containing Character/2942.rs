// Time complexity: O(nm)
// Space complexity: O(k)
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, word)| {
                if word.contains(x) {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}
