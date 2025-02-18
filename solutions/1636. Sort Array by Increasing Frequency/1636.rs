// Time complexity: O(sort)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            *freq.entry(num).or_default() += 1;
        }

        nums.sort_unstable_by(|&a, &b| {
            if freq[&a] == freq[&b] {
                b.cmp(&a)
            } else {
                freq[&a].cmp(&freq[&b])
            }
        });

        nums
    }
}
