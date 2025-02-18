// Time Complexity: O(n)
// Space Complexity: O(n)
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = n + 1;
        let mut dq = VecDeque::new();
        let mut prefix = vec![0i64; n + 1];

        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        for i in 0..=n {
            while let Some(&front) = dq.front() {
                if prefix[i] - prefix[front] >= k as i64 {
                    ans = ans.min(i - front);
                    dq.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&back) = dq.back() {
                if prefix[i] <= prefix[back] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back(i);
        }

        if ans <= n {
            ans as i32
        } else {
            -1
        }
    }
}
