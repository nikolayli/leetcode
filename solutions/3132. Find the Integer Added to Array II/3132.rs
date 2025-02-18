// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut ans = std::i32::MAX;

        for i in 0..nums1.len() {
            let x = nums2[0] - nums1[i];
            if Self::check(&nums1, &nums2, x) {
                ans = ans.min(x);
            }
        }

        ans
    }

    fn check(nums1: &Vec<i32>, nums2: &Vec<i32>, x: i32) -> bool {
        let mut count = 0;
        let mut j = 0;

        for i in 0..nums1.len() {
            if j < nums2.len() {
                if nums1[i] + x != nums2[j] {
                    count += 1;
                } else {
                    j += 1;
                }
            } else {
                break;
            }
        }

        count <= 2
    }
}
