// Time complexity: O(nlogk)
// Space complexity: O(k)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut min_heap = BinaryHeap::new();
        let mut max_range = i32::MIN;

        for (i, row) in nums.iter().enumerate() {
            min_heap.push(Reverse((row[0], i, 0)));
            max_range = max_range.max(row[0]);
        }

        let mut ans = vec![min_heap.peek().unwrap().0 .0, max_range];

        while min_heap.len() == nums.len() {
            let Reverse((num, r, c)) = min_heap.pop().unwrap();
            if c + 1 < nums[r].len() {
                min_heap.push(Reverse((nums[r][c + 1], r, c + 1)));
                max_range = max_range.max(nums[r][c + 1]);
                let min_range = min_heap.peek().unwrap().0 .0;
                if max_range - min_range < ans[1] - ans[0] {
                    ans[0] = min_range;
                    ans[1] = max_range;
                }
            }
        }

        ans
    }
}
