// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0_i64;

        nums.sort_unstable();

        let mut l = 0;
        for r in 0..nums.len() {
            sum += nums[r] as i64;
            while sum + (k as i64) < nums[r] as i64 * (r as i64 - l as i64 + 1) {
                sum -= nums[l] as i64;
                l += 1;
            }
            ans = ((r - l + 1) as i32).max(ans);
        }

        ans
    }
}
