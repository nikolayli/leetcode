// Time complexity: O(m+n)
// Space complexity: O(m+n)
use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut count = HashMap::new();
        let text = format!("{} {}", s1, s2);

        for word in text.split_whitespace() {
            *count.entry(word.to_string()).or_insert(0) += 1;
        }

        for (word, freq) in count {
            if freq == 1 {
                ans.push(word);
            }
        }

        ans
    }
}
