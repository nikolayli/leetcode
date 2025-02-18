// Time complexity: O(nk)
// Space complexity: O(n)
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn is_isomorphic(w: &str, p: &str) -> bool {
            w.chars().map(|c| w.find(c)).collect::<Vec<_>>()
                == p.chars().map(|c| p.find(c)).collect::<Vec<_>>()
        }

        words
            .into_iter()
            .filter(|word| is_isomorphic(word, &pattern))
            .collect()
    }
}
