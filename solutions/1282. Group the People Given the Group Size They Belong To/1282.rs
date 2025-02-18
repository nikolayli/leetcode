// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut group_size_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &group_size) in group_sizes.iter().enumerate() {
            group_size_to_indices
                .entry(group_size)
                .or_insert(Vec::new())
                .push(i);
        }

        for (group_size, indices) in group_size_to_indices.iter() {
            let mut group_indices: Vec<i32> = Vec::new();
            for &index in indices {
                group_indices.push(index as i32);
                if group_indices.len() == *group_size as usize {
                    ans.push(group_indices.clone());
                    group_indices.clear();
                }
            }
        }

        ans
    }
}
