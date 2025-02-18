// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();

        fn count_less(nums: &[i32], sum: i32) -> i64 {
            let mut res = 0;
            let mut i = 0;
            let mut j = nums.len() as i32 - 1;
            while i < j {
                while i < j && nums[i as usize] + nums[j as usize] > sum {
                    j -= 1;
                }
                res += (j - i) as i64;
                i += 1;
            }
            res
        }

        count_less(&nums, upper) - count_less(&nums, lower - 1)
    }
}
