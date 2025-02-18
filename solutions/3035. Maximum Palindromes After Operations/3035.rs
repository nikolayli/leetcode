// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut count = HashMap::new();

        for word in &words {
            for c in word.chars() {
                *count.entry(c).or_insert(0) += 1;
            }
        }

        let mut pairs = count.values().map(|&v| v / 2).sum::<i32>();
        let mut lengths: Vec<usize> = words.iter().map(|word| word.len()).collect();
        lengths.sort_unstable();

        for &length in &lengths {
            let need_pairs = (length / 2) as i32;

            if pairs < need_pairs {
                return ans;
            }

            ans += 1;
            pairs -= need_pairs;
        }

        ans
    }
}
