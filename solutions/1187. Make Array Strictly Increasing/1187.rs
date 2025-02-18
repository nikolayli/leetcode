// Time complexity : O(sort)
// Space complexity: O(|arr1|)
use std::collections::BTreeMap;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        let mut dp = BTreeMap::new();
        dp.insert(-1, 0);

        arr2.sort_unstable();
        arr2.dedup();

        for &a in &arr1 {
            let mut next_dp = BTreeMap::new();
            for (&val, &steps) in &dp {
                if a > val {
                    let entry = next_dp.entry(a).or_insert(i32::MAX);
                    *entry = (*entry).min(steps);
                }
                let i = match arr2.binary_search(&val) {
                    Ok(idx) => idx + 1,
                    Err(idx) => idx,
                };
                if i < arr2.len() {
                    let entry = next_dp.entry(arr2[i]).or_insert(i32::MAX);
                    *entry = (*entry).min(steps + 1);
                }
            }
            if next_dp.is_empty() {
                return -1;
            }
            dp = next_dp;
        }

        *dp.values().min().unwrap_or(&i32::MAX)
    }
}
