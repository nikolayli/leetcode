// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut count = HashMap::new();
        let mut k = k as usize;

        for a in &arr {
            *count.entry(a).or_insert(0) += 1;
        }

        for a in &arr {
            if count.get(a) == Some(&1) {
                k -= 1;
                if k == 0 {
                    return a.clone();
                }
            }
        }

        String::new()
    }
}
