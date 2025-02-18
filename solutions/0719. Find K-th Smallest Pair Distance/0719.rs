// Time complexity: O(sort)+O(nlog(max(nums)−min(nums)))
// Space complexity: O(sort)
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut l = 0;
        let mut r = nums[nums.len() - 1] - nums[0];

        while l < r {
            let m = l + (r - l) / 2;
            if Self::count_pairs_less_equal(&nums, m) >= k as usize {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }

    fn count_pairs_less_equal(nums: &Vec<i32>, m: i32) -> usize {
        let mut count = 0;
        let mut j = 0;

        for i in 0..nums.len() {
            while j < nums.len() && nums[j] <= nums[i] + m {
                j += 1;
            }
            count += j - i - 1;
        }

        count
    }
}
