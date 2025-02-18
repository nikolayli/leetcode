// Time complexity: O(n*2^n*m)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for &letter in &letters {
            *count.entry(letter).or_insert(0) += 1;
        }

        fn use_word(word: &str, count: &mut HashMap<char, i32>, score: &[i32]) -> i32 {
            let mut is_valid = true;
            let mut earned = 0;

            for c in word.chars() {
                let entry = count.entry(c).or_insert(0);
                *entry -= 1;
                if *entry < 0 {
                    is_valid = false;
                }
                earned += score[c as usize - 'a' as usize];
            }

            if is_valid {
                earned
            } else {
                -1
            }
        }

        fn unuse_word(word: &str, count: &mut HashMap<char, i32>) {
            for c in word.chars() {
                *count.entry(c).or_insert(0) += 1;
            }
        }

        fn dfs(words: &[String], s: usize, count: &mut HashMap<char, i32>, score: &[i32]) -> i32 {
            let mut ans = 0;

            for i in s..words.len() {
                let earned = use_word(&words[i], count, score);
                if earned > 0 {
                    ans = ans.max(earned + dfs(&words, i + 1, count, score));
                }
                unuse_word(&words[i], count);
            }

            ans
        }

        dfs(&words, 0, &mut count, &score)
    }
}
