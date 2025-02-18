// Time complexity: O(nlog(max(nums)))
// Space complexity: O(1)
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        fn num_operations(nums: &Vec<i32>, m: i32) -> i32 {
            nums.iter().map(|&num| (num - 1) / m).sum()
        }

        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();

        while l < r {
            let m = l + (r - l) / 2;
            if num_operations(&nums, m) <= max_operations {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}
