// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut min_heap: BinaryHeap<Reverse<i32>> = count.values().map(|&v| Reverse(v)).collect();

        for a in arr {
            *count.entry(a).or_insert(0) += 1;
        }

        while k > 0 {
            if let Some(Reverse(top)) = min_heap.pop() {
                k -= top;
            }
        }

        min_heap.len() as i32 + if k < 0 { 1 } else { 0 }
    }
}
