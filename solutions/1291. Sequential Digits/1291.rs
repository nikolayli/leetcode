// Time complexity: O(loghigh)
// Space complexity: O(loghigh)
use std::collections::VecDeque;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut q: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        while let Some(num) = q.pop_front() {
            if num > high {
                return ans;
            }
            if low <= num && num <= high {
                ans.push(num);
            }
            let last_digit = num % 10;
            if last_digit < 9 {
                q.push_back(num * 10 + last_digit + 1);
            }
        }

        ans
    }
}
