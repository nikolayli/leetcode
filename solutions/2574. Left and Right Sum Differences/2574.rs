// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();

        for &num in &nums {
            right_sum -= num;
            ans.push((left_sum - right_sum).abs());
            left_sum += num;
        }

        ans
    }
}
