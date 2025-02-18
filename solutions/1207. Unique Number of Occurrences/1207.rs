// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts = HashMap::new();
        arr.into_iter()
            .for_each(|num| *counts.entry(num).or_insert(0) += 1);
        counts.values().collect::<HashSet<_>>().len() == counts.len()
    }
}
