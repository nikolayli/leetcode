// Time complexity: O(sort)
// Space complexity: O(n)
use std::cmp::Reverse;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut height_and_names: Vec<(i32, String)> =
            heights.into_iter().zip(names.into_iter()).collect();
        height_and_names.sort_unstable_by_key(|&(height, _)| Reverse(height));

        height_and_names.into_iter().map(|(_, name)| name).collect()
    }
}
