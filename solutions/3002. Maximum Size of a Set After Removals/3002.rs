// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set1: HashSet<_> = nums1.iter().cloned().collect();
        let set2: HashSet<_> = nums2.iter().cloned().collect();
        let n = nums1.len();
        let n1 = set1.len();
        let n2 = set2.len();
        let nc = set1.intersection(&set2).count();
        let max_unique_nums1 = std::cmp::min(n1 - nc, n / 2);
        let max_unique_nums2 = std::cmp::min(n2 - nc, n / 2);

        std::cmp::min(n, max_unique_nums1 + max_unique_nums2 + nc) as i32
    }
}
