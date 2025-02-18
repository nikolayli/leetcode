// Time complexity: O(logn + m)
// Space complexity: O(1)
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let mut ans = 0;
        let mut i = 0;
        let mut miss: i64 = 1;

        while miss <= n {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                ans += 1;
            }
        }

        ans
    }
}
