// Time complexity: O(m+n)
// Space complexity: O(min(m,n))
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Self::intersect(nums2, nums1);
        }

        let mut ans = Vec::new();
        let mut count = HashMap::new();

        for &num in &nums1 {
            *count.entry(num).or_insert(0) += 1;
        }

        for &num in &nums2 {
            if let Some(c) = count.get_mut(&num) {
                if *c > 0 {
                    ans.push(num);
                    *c -= 1;
                }
            }
        }

        ans
    }
}
