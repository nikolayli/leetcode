// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let k = k as usize;
        let candidates = candidates as usize;
        let mut ans: i64 = 0;
        let mut i = 0;
        let mut j = costs.len() as isize - 1;
        let mut min_heap_l = BinaryHeap::new();
        let mut min_heap_r = BinaryHeap::new();

        for _ in 0..k {
            while min_heap_l.len() < candidates && i as isize <= j {
                min_heap_l.push(Reverse(costs[i]));
                i += 1;
            }
            while min_heap_r.len() < candidates && i as isize <= j {
                min_heap_r.push(Reverse(costs[j as usize]));
                j -= 1;
            }
            if min_heap_l.is_empty() {
                ans += min_heap_r.pop().unwrap().0 as i64;
            } else if min_heap_r.is_empty() {
                ans += min_heap_l.pop().unwrap().0 as i64;
            } else if min_heap_l.peek().unwrap().0 <= min_heap_r.peek().unwrap().0 {
                ans += min_heap_l.pop().unwrap().0 as i64;
            } else {
                ans += min_heap_r.pop().unwrap().0 as i64;
            }
        }

        ans
    }
}
