// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_num = nums.iter().max().unwrap();
        let mut ans = 0;
        let mut count = 0;

        let mut l = 0;
        for r in 0..nums.len() {
            if (nums[r] == *max_num) {
                count += 1;
            }
            while count == k {
                if nums[l] == *max_num {
                    count -= 1;
                }
                l += 1;
            }
            ans += l as i64;
        }

        ans
    }
}
