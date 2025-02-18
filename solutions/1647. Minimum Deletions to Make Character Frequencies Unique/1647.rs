// Time complexity: O(n)
// Space complexity: O(1)
use std::collections::HashSet;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut ans = 0;
        let mut freq = vec![0; 26];
        let mut used_freq = HashSet::new();

        for c in s.chars() {
            freq[(c as usize) - ('a' as usize)] += 1;
        }

        for &f in freq.iter() {
            let mut f = f;
            while f > 0 && used_freq.contains(&f) {
                f -= 1;
                ans += 1;
            }
            used_freq.insert(f);
        }

        ans
    }
}
