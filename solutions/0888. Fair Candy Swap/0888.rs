// Time complexity: O(n+m)
// Space complexity: O(m)
use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let diff = (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>()) / 2;
        let bob_sizes_set: HashSet<_> = bob_sizes.iter().collect();

        for alice_size in alice_sizes {
            let target = alice_size - diff;
            if bob_sizes_set.contains(&target) {
                return vec![alice_size, target];
            }
        }

        panic!("No fair candy swap found")
    }
}
