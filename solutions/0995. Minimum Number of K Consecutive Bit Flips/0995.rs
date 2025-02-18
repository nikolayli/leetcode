// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut flipped_time = 0;
        let k = k as usize;

        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                flipped_time -= 1;
            }
            if flipped_time % 2 == nums[i] {
                if i + k > nums.len() {
                    return -1;
                }
                ans += 1;
                flipped_time += 1;
                nums[i] = 2;
            }
        }

        ans
    }
}
