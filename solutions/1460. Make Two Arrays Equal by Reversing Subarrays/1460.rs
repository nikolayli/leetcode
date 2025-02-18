// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for num in arr {
            *count.entry(num).or_insert(0) += 1;
        }

        for num in target {
            match count.get_mut(&num) {
                Some(c) => {
                    *c -= 1;
                    if *c == 0 {
                        count.remove(&num);
                    }
                }
                None => return false,
            }
        }

        count.is_empty()
    }
}
