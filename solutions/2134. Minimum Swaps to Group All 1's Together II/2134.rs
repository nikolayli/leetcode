// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k = nums.iter().filter(|&&x| x == 1).count();
        let mut ones = 0;
        let mut max_ones = 0;

        for i in 0..(n * 2) {
            if i >= k && nums[(i - k) % n] == 1 {
                ones -= 1;
            }
            if nums[i % n] == 1 {
                ones += 1;
            }
            max_ones = max_ones.max(ones);
        }

        (k - max_ones) as i32
    }
}
