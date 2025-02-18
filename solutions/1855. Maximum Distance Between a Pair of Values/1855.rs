// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j] {
                i += 1;
            }
            j += 1;
        }

        if i == j {
            0
        } else {
            (j - i - 1) as i32
        }
    }
}
