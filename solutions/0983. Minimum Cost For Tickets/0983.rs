// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::VecDeque;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last7 = VecDeque::new();
        let mut last30 = VecDeque::new();

        for &day in &days {
            while let Some(&(d, _)) = last7.front() {
                if d + 7 <= day {
                    last7.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&(d, _)) = last30.front() {
                if d + 30 <= day {
                    last30.pop_front();
                } else {
                    break;
                }
            }
            last7.push_back((day, ans + costs[1]));
            last30.push_back((day, ans + costs[2]));
            ans = *[
                ans + costs[0],
                last7.front().unwrap().1,
                last30.front().unwrap().1,
            ]
            .iter()
            .min()
            .unwrap();
        }

        ans
    }
}
