// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let unique_nums: HashSet<i64> = nums.iter().cloned().map(|x| x as i64).collect();

        for &num in &nums {
            let mut curr_streak = 0;
            let mut curr = num as i64;

            while unique_nums.contains(&curr) {
                curr_streak += 1;
                if curr > 100_000 {
                    break;
                }
                curr *= curr;
            }
            ans = ans.max(curr_streak);
        }

        if ans < 2 {
            -1
        } else {
            ans
        }
    }
}
