// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::VecDeque;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let n = s.len();
        let mut ans = String::new();
        let mut count = vec![0; 128];
        let mut buckets = vec![VecDeque::new(); n + 1];

        for c in s.chars() {
            count[c as usize] += 1;
        }

        for i in 0..128 {
            let freq = count[i];
            if freq > 0 {
                buckets[freq].push_back(i as u8 as char);
            }
        }

        for freq in (1..=n).rev() {
            for c in buckets[freq].iter() {
                ans += &c.to_string().repeat(freq);
            }
        }

        ans
    }
}
