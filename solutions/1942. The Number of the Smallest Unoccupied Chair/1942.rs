// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut times = times;
        let mut next_unsat_chair = 0;
        let mut empty_chairs = BinaryHeap::new();
        let mut occupied = BinaryHeap::new();

        for (i, time) in times.iter_mut().enumerate() {
            time.push(i as i32);
        }

        times.sort_unstable();

        for time in times {
            let arrival = time[0];
            let leaving = time[1];
            let i = time[2];

            while let Some(Reverse((leave_time, chair))) = occupied.peek() {
                if *leave_time <= arrival {
                    empty_chairs.push(Reverse(*chair));
                    occupied.pop();
                } else {
                    break;
                }
            }

            if i == target_friend {
                return if let Some(Reverse(chair)) = empty_chairs.peek() {
                    *chair
                } else {
                    next_unsat_chair
                };
            }

            if let Some(Reverse(chair)) = empty_chairs.pop() {
                occupied.push(Reverse((leaving, chair)));
            } else {
                occupied.push(Reverse((leaving, next_unsat_chair)));
                next_unsat_chair += 1;
            }
        }

        unreachable!();
    }
}
