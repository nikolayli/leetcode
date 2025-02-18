// Time complexity: O(2^n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn dfs(s: &str, start: usize, seen: &mut HashSet<String>, ans: &mut usize) {
            if start == s.len() {
                *ans = (*ans).max(seen.len());
                return;
            }
            for i in 1..=s.len() - start {
                let cand = &s[start..start + i];
                if seen.contains(cand) {
                    continue;
                }
                seen.insert(cand.to_string());
                dfs(s, start + i, seen, ans);
                seen.remove(cand);
            }
        }

        let mut ans = 0;
        dfs(&s, 0, &mut HashSet::new(), &mut ans);
        ans as i32
    }
}
