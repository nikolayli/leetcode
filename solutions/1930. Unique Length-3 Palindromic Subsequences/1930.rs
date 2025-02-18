// Time complexity: O(n)
// Space complexity: O(1)
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut ans = 0;
        let mut first = vec![s.len(); 26];
        let mut last = vec![0; 26];

        for (i, &c) in s.as_bytes().iter().enumerate() {
            let index = (c - b'a') as usize;
            first[index] = first[index].min(i);
            last[index] = i;
        }

        for (&f, &l) in first.iter().zip(last.iter()) {
            if f < l {
                let unique_chars: HashSet<u8> = s[f + 1..l].as_bytes().iter().cloned().collect();
                ans += unique_chars.len() as i32;
            }
        }

        ans
    }
}
