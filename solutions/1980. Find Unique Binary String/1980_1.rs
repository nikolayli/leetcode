// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let bit_size = nums[0].len();
        let max_num = 1 << bit_size;
        let mut nums_set = HashSet::new();

        for num in nums {
            nums_set.insert(i32::from_str_radix(&num, 2).unwrap());
        }

        for num in 0..max_num {
            if !nums_set.contains(&num) {
                return format!("{:0width$b}", num, width = bit_size);
            }
        }

        panic!()
    }
}
