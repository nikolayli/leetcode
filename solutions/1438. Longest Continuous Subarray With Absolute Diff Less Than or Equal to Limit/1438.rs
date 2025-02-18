// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut min_q: VecDeque<i32> = VecDeque::new();
        let mut max_q: VecDeque<i32> = VecDeque::new();

        nums.iter()
            .enumerate()
            .fold((0, 0, 1), |(mut l, mut r, mut ans), (i, &num)| {
                while min_q.back().map_or(false, |&back| back > num) {
                    min_q.pop_back();
                }
                min_q.push_back(num);

                while max_q.back().map_or(false, |&back| back < num) {
                    max_q.pop_back();
                }
                max_q.push_back(num);

                while max_q.front().unwrap() - min_q.front().unwrap() > limit {
                    if *min_q.front().unwrap() == nums[l] {
                        min_q.pop_front();
                    }
                    if *max_q.front().unwrap() == nums[l] {
                        max_q.pop_front();
                    }
                    l += 1;
                }
                ans = ans.max(i as i32 - l as i32 + 1);
                (l, r + 1, ans)
            })
            .2
    }
}
