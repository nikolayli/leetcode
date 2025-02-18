// Time complexity: O(n)
// Space complexity: O(1)
use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        const VOWELS: &str = "aeiou";
        let mut ans = 0;
        let mut prefix = 0;
        let mut prefix_to_index = HashMap::new();
        prefix_to_index.insert(0, -1);

        for (i, c) in s.chars().enumerate() {
            if let Some(index) = VOWELS.find(c) {
                prefix ^= 1 << index;
            }
            if !prefix_to_index.contains_key(&prefix) {
                prefix_to_index.insert(prefix, i as i32);
            }
            ans = ans.max(i as i32 - prefix_to_index[&prefix]);
        }

        ans
    }
}
