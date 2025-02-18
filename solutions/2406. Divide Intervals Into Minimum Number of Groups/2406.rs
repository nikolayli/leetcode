// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable();

        let mut min_heap = BinaryHeap::new();

        for interval in intervals {
            if let Some(&Reverse(top)) = min_heap.peek() {
                if interval[0] > top {
                    min_heap.pop();
                }
            }
            min_heap.push(Reverse(interval[1]));
        }

        min_heap.len() as i32
    }
}
