// Time complexity: O(10^4)
// Space complexity: O(10^4)
use std::collections::{HashSet, VecDeque};

impl Solution {
    fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut seen: HashSet<String> = deadends.into_iter().collect();
        if seen.contains("0000") {
            return -1;
        }
        if target == "0000" {
            return 0;
        }

        let mut ans = 0;
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back("0000".to_string());

        while !queue.is_empty() {
            ans += 1;
            for _ in 0..queue.len() {
                let word = queue.pop_front().unwrap();
                for i in 0..4 {
                    let mut chars: Vec<char> = word.chars().collect();
                    let cache = chars[i];
                    chars[i] = if chars[i] == '9' {
                        '0'
                    } else {
                        (chars[i] as u8 + 1) as char
                    };
                    let new_word: String = chars.iter().collect();
                    if new_word == target {
                        return ans;
                    }
                    if !seen.contains(&new_word) {
                        queue.push_back(new_word.clone());
                        seen.insert(new_word);
                    }
                    chars[i] = cache;

                    chars[i] = if chars[i] == '0' {
                        '9'
                    } else {
                        (chars[i] as u8 - 1) as char
                    };
                    let new_word: String = chars.iter().collect();
                    if new_word == target {
                        return ans;
                    }
                    if !seen.contains(&new_word) {
                        queue.push_back(new_word.clone());
                        seen.insert(new_word);
                    }
                }
            }
        }

        -1
    }
}
