// Time complexity: O(nlogn+k)
// Space complexity: O(n+k)
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut mod_to_subset = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for &num in &nums {
            mod_to_subset
                .entry(num % k)
                .or_insert_with(HashSet::new)
                .insert(num);
        }

        let mut prev_num = -k;
        let (mut skip, mut pick) = (0, 0);

        for subset in mod_to_subset.values() {
            let mut sorted_subset: Vec<_> = subset.iter().cloned().collect();
            sorted_subset.sort_unstable();

            for &num in &sorted_subset {
                let non_empty_count = (1 << count[&num]) - 1;
                let new_pick =
                    non_empty_count * (1 + skip + if num - prev_num == k { 0 } else { pick });
                skip += pick;
                pick = new_pick;
                prev_num = num;
            }
        }

        skip + pick
    }
}
