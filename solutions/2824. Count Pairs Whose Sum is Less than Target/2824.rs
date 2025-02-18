// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] < target {
                    ans += 1;
                }
            }
        }

        ans
    }
}
