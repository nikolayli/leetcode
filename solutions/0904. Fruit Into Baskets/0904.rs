// Time Complexity: O(n)
// Space Complexity: O(1)
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = HashMap::new();
        let mut l = 0;

        for r in 0..fruits.len() {
            *count.entry(fruits[r]).or_insert(0) += 1;

            while count.len() > 2 {
                let left_fruit = fruits[l];
                if let Some(x) = count.get_mut(&left_fruit) {
                    *x -= 1;
                    if *x == 0 {
                        count.remove(&left_fruit);
                    }
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}
