// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut sum = 0;

        for &num in &nums {
            if num % 6 == 0 {
                sum += num;
                count += 1;
            }
        }

        if count == 0 {
            0
        } else {
            sum / count
        }
    }
}
