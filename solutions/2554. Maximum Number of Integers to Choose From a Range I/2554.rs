// Time complexity: O(n+m)
// Space complexity: O(m)
use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0;
        let banned_set: HashSet<i32> = banned.into_iter().collect();

        for i in 1..=n {
            if !banned_set.contains(&i) && sum + i <= max_sum {
                ans += 1;
                sum += i;
            }
        }

        ans
    }
}
