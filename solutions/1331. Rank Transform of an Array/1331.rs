// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut rank = HashMap::new();
        let mut sorted_arr: Vec<_> = arr.clone();
        sorted_arr.sort_unstable();

        for &a in sorted_arr.iter() {
            if !rank.contains_key(&a) {
                rank.insert(a, rank.len() as i32 + 1);
            }
        }

        arr.into_iter().map(|a| *rank.get(&a).unwrap()).collect()
    }
}
