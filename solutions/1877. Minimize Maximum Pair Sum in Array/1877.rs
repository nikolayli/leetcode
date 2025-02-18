// Time complexity: O(nsort)
// Space complexity: O(sort)
impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        nums.sort();

        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            ans = ans.max(nums[i] + nums[j]);
            i += 1;
            j -= 1;
        }
        ans
    }
}
