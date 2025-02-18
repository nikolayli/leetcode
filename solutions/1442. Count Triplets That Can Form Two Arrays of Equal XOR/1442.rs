// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pref = 0;
        let mut count_map: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        let mut total_map: HashMap<i32, i32> = HashMap::new();

        for (i, &val) in arr.iter().enumerate() {
            pref ^= val;
            ans +=
                count_map.get(&pref).unwrap_or(&0) * i as i32 - total_map.get(&pref).unwrap_or(&0);
            *count_map.entry(pref).or_insert(0) += 1;
            *total_map.entry(pref).or_insert(0) += i as i32 + 1;
        }

        ans
    }
}
