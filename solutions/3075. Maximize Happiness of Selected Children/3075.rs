// Time complexity: O(sort)
// Space complexity: O(sort)
use std::cmp;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut ans: i64 = 0;
        let mut decremented = 0;

        happiness.sort_unstable_by(|a, b| b.cmp(a));

        for i in 0..k {
            ans += cmp::max(0, happiness[i] - decremented) as i64;
            decremented += 1;
        }

        ans
    }
}
