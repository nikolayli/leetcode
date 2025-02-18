// Time complexity: O(2^n)
// Space complexity: O(2^n)
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let mut mem: HashMap<String, Vec<String>> = HashMap::new();

        Self::break_word(&s, &word_set, &mut mem)
    }

    fn break_word(
        s: &str,
        word_set: &HashSet<String>,
        mem: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        if let Some(m) = mem.get(s) {
            return m.clone();
        }

        let mut ans = Vec::new();

        if word_set.contains(s) {
            ans.push(s.to_string());
        }

        for i in 1..s.len() {
            let prefix = &s[..i];
            if word_set.contains(prefix) {
                let suffix = &s[i..];
                for word in Self::break_word(suffix, word_set, mem).iter() {
                    ans.push(format!("{} {}", prefix, word));
                }
            }
        }

        mem.insert(s.to_string(), ans.clone());

        ans
    }
}
